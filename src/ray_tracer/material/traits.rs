use crate::ray_tracer::{
    common::vec3::structs::Color,
    render::{hit::hit_record::HitRecord, ray::structs::Ray},
};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Option<Color>, Option<Ray>);
}
