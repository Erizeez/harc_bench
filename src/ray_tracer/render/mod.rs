use std::sync::{Arc, RwLock};

use indicatif::ProgressBar;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{Buffer, MAX_DEPTH, SAMPLES_PER_PIXEL};

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
    world: Arc<RwLock<HittableList>>,
    camera: Camera,
    bar: &ProgressBar,
    buffer: Buffer,
) {
    (0..image_height).into_par_iter().rev().for_each(|j| {
        (0..image_width).into_par_iter().for_each(|i| {
            let pixel_color = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    ray_color(r, &world.read().unwrap(), MAX_DEPTH)
                })
                .fold(Color::new(0.0, 0.0, 0.0), |acc, x| acc + x);
            bar.inc(1);

            buffer.get()[(image_height - j - 1) as usize * image_width as usize + i as usize] +=
                <Color as Into<u32>>::into(pixel_color / SAMPLES_PER_PIXEL as f64);
        })
    })
}
