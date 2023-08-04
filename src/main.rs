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
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 20;
const DEFAULT_MAX_DEPTH: u32 = 50;
const DEFAULT_VFOV: f64 = 20.0;

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

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

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

    let image_width = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_IMAGE_WIDTH);

    let aspect_ratio = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ASPECT_RATIO);

    let samples_per_pixel = std::env::args()
        .nth(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SAMPLES_PER_PIXEL);

    let max_depth = std::env::args()
        .nth(4)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MAX_DEPTH);

    let vfov = std::env::args()
        .nth(5)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_VFOV);

    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
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
        vfov,
        aspect_ratio,
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

    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg} ETA [{eta_precise}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    main_render(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        world,
        camera,
        &bar,
        buffer,
    );

    bar.finish();

    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    let milliseconds = duration.as_millis();
    println!("Score: {:?}", pixels_account * 1000 / milliseconds);

    loop {}
}
