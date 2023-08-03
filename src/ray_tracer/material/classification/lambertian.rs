use crate::ray_tracer::{
    common::vec3::{random::random_unit_vector, structs::Color},
    material::traits::Material,
    render::{hit::hit_record::HitRecord, ray::structs::Ray},
};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> (bool, Option<Color>, Option<Ray>) {
        let mut scatter_direction = rec.normal.clone() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        (
            true,
            Some(self.albedo.clone()),
            Some(Ray::new(rec.p.clone(), scatter_direction)),
        )
    }
}
