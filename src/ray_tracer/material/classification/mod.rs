use crate::ray_tracer::{
    common::vec3::structs::Color,
    render::{hit::hit_record::HitRecord, ray::structs::Ray},
};

use super::traits::Material;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct NullMaterial {}

impl NullMaterial {
    pub fn new() -> Self {
        NullMaterial {}
    }
}

impl Material for NullMaterial {
    fn scatter(&self, _r_in: Ray, _rec: HitRecord) -> (bool, Option<Color>, Option<Ray>) {
        todo!()
    }
}
