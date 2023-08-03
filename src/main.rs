use std::{
    io::Write,
    path::Path,
    sync::{Arc, RwLock},
};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

pub mod ray_tracer;

use ray_tracer::{
    camera::Camera,
    common::{
        export::write_color,
        extra::{random_double, random_double_range},
        vec3::structs::{Color, Point3, Vec3},
    },
    material::{
        classification::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
        traits::Material,
    },
    object::sphere::Sphere,
    render::hit::hittable_list::HittableList,
};

use crate::ray_tracer::render::ray::utils::ray_color;

const DEFAULT_IMAGE_WIDTH: u32 = 3840;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAX_DEPTH: u32 = 50;

fn main() {
    let start = std::time::Instant::now();
    // Receive a interger and has a default value of 4
    let image_width = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_IMAGE_WIDTH);

    let image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    let pixels_account: u128 = image_width as u128 * image_height as u128;

    let world = Arc::new(RwLock::new(HittableList::new()));

    // random scene
    let material_ground = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric { ir: 1.5 });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

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

    let mut file = std::fs::File::create("temp/image.ppm").unwrap();
    let mut contents = String::new();

    contents.push_str("P3\n");
    contents.push_str(&format!("{} {}\n", image_width, image_height));
    contents.push_str("255\n");

    let mut outer_contents = Vec::<String>::new();

    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:80.cyan/blue} {pos:>7}/{len:7} {msg} ETA [{eta_precise}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            // println!("Scanlines remaining: {}", j);
            let mut inter_contents = Vec::<String>::new();
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let pixel_color = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                            let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                            let r = camera.get_ray(u, v);
                            ray_color(r, &world.read().unwrap(), MAX_DEPTH)
                        })
                        .fold(Color::new(0.0, 0.0, 0.0), |acc, x| acc + x);
                    bar.inc(1);
                    write_color(pixel_color, SAMPLES_PER_PIXEL)
                })
                .collect_into_vec(&mut inter_contents);
            inter_contents.join("")
        })
        .collect_into_vec(&mut outer_contents);

    contents.push_str(outer_contents.join("").as_str());
    bar.finish();

    file.write_all(contents.as_bytes()).unwrap();

    // println!("Done.");

    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    let milliseconds = duration.as_millis();
    println!("Score: {:?}", pixels_account * 1000 / milliseconds);

    let img = image::open("temp/image.ppm").expect("Failed to open image");

    // 将图像保存为 PNG 文件
    img.save(Path::new("temp/image.png"))
        .expect("Failed to save image");
}
