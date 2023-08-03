use crate::ray_tracer::{
    common::{
        extra::random_double,
        vec3::structs::{Color, Vec3},
    },
    material::traits::Material,
    render::{hit::hit_record::HitRecord, ray::structs::Ray},
};

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv.clone()).dot(n.clone()).min(1.0);
    let r_out_perp = etai_over_etat * (uv.clone() + cos_theta * n.clone());
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n.clone();
    r_out_perp + r_out_parallel
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn dielectric(ir: f64) -> Self {
        Dielectric { ir }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Option<Color>, Option<Ray>) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (-unit_direction.clone()).dot(rec.normal.clone()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                unit_direction.reflect(rec.normal.clone())
            } else {
                refract(unit_direction.clone(), rec.normal.clone(), refraction_ratio)
            };

        let scattered = Ray::new(rec.p.clone(), direction);
        (true, Some(attenuation), Some(scattered))
    }
}
