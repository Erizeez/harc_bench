use std::sync::Arc;

use crate::ray_tracer::{
    common::vec3::structs::Point3,
    material::traits::Material,
    render::{
        hit::{hit_record::HitRecord, traits::Hittable},
        ray::structs::Ray,
    },
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let oc = r.origin() - self.center.clone();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, None);
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return (false, None);
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = self.material.clone();

        (true, Some(rec))
    }
}
