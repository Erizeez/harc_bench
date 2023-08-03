use crate::ray_tracer::render::ray::structs::Ray;

use super::hit_record::HitRecord;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>);
}
