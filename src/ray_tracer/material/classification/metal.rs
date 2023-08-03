use crate::ray_tracer::{
    common::vec3::{random::random_in_unit_sphere, structs::Color},
    material::traits::Material,
    render::{hit::hit_record::HitRecord, ray::structs::Ray},
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 0.0 }
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Option<Color>, Option<Ray>) {
        let reflected = r_in.direction().unit_vector().reflect(rec.normal.clone());
        let scattered: Ray = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        (
            scattered.direction().dot(rec.normal.clone()) > 0.0,
            Some(self.albedo.clone()),
            Some(scattered),
        )
    }
}
