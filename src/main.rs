use indicatif::{ProgressBar, ProgressStyle};
use minifb::{Key, Window, WindowOptions};

pub mod ray_tracer;

use ray_tracer::{
    camera::Camera,
    common::{
        buffer::Buffer,
        vec3::structs::{Point3, Vec3},
    },
};
use tokio::runtime::Runtime;

use crate::ray_tracer::{render::main_render, scene::example_1::generate_example_scene_1};

const DEFAULT_IMAGE_WIDTH: u32 = 3840;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_DEPTH: u32 = 50;

// A func recv from Channel and loop
async fn pixel_receiver(width: usize, height: usize, buffer: Buffer) {
    // create a window
    // let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Real-time Pixel Modification",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let shared_buffer = &mut *buffer.get();

        window
            .update_with_buffer(&shared_buffer, width, height)
            .unwrap();
    }

    // Exit the whole program
    std::process::exit(0);
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    // Receive a interger and has a default value of 4
    let image_width = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_IMAGE_WIDTH);

    let image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    let pixels_account: u128 = image_width as u128 * image_height as u128;

    let world = generate_example_scene_1();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let buffer = Buffer::new(image_width as usize, image_height as usize);

    let rt = Runtime::new().unwrap();
    rt.spawn(pixel_receiver(
        image_width as usize,
        image_height as usize,
        buffer.clone(),
    ));

    // let mut file = std::fs::File::create("temp/image.ppm").unwrap();
    // let mut contents = String::new();

    // contents.push_str("P3\n");
    // contents.push_str(&format!("{} {}\n", image_width, image_height));
    // contents.push_str("255\n");

    // let mut outer_contents = Vec::<String>::new();

    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg} ETA [{eta_precise}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    main_render(image_width, image_height, world, camera, &bar, buffer);

    // (0..image_height)
    //     .into_par_iter()
    //     .rev()
    //     .map(|j| {
    //         // println!("Scanlines remaining: {}", j);
    //         let mut inter_contents = Vec::<String>::new();
    //         (0..image_width)
    //             .into_par_iter()
    //             .map(|i| {
    //                 let pixel_color = (0..SAMPLES_PER_PIXEL)
    //                     .map(|_| {
    //                         let u = (i as f64 + random_double()) / (image_width - 1) as f64;
    //                         let v = (j as f64 + random_double()) / (image_height - 1) as f64;
    //                         let r = camera.get_ray(u, v);
    //                         ray_color(r, &world.read().unwrap(), MAX_DEPTH)
    //                     })
    //                     .fold(Color::new(0.0, 0.0, 0.0), |acc, x| acc + x);
    //                 bar.inc(1);
    //                 write_color(pixel_color, SAMPLES_PER_PIXEL)
    //             })
    //             .collect_into_vec(&mut inter_contents);
    //         inter_contents.join("")
    //     })
    //     .collect_into_vec(&mut outer_contents);

    // contents.push_str(outer_contents.join("").as_str());
    bar.finish();

    // file.write_all(contents.as_bytes()).unwrap();

    // println!("Done.");

    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    let milliseconds = duration.as_millis();
    println!("Score: {:?}", pixels_account * 1000 / milliseconds);

    loop {}

    // let img = image::open("temp/image.ppm").expect("Failed to open image");

    // // 将图像保存为 PNG 文件
    // img.save(Path::new("temp/image.png"))
    //     .expect("Failed to save image");
}
