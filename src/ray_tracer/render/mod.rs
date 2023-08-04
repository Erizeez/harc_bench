use std::sync::{Arc, RwLock};

use indicatif::ProgressBar;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::Buffer;

use self::{hit::hittable_list::HittableList, ray::utils::ray_color};

use super::{
    camera::Camera,
    common::{extra::random_double, vec3::structs::Color},
};

pub mod hit;
pub mod ray;

pub fn main_render(
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    world: Arc<RwLock<HittableList>>,
    camera: Camera,
    bar: &ProgressBar,
    buffer: Buffer,
) {
    (0..image_height).into_par_iter().rev().for_each(|j| {
        (0..image_width).into_par_iter().for_each(|i| {
            let pixel_color = (0..samples_per_pixel)
                .map(|_| {
                    let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    ray_color(r, &world.read().unwrap(), max_depth)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |acc, x| acc + x);
            bar.inc(1);

            buffer.get()[(image_height - j - 1) as usize * image_width as usize + i as usize] +=
                <Color as Into<u32>>::into(pixel_color / samples_per_pixel as f64);
        })
    })
}
