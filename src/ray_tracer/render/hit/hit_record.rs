use std::sync::Arc;

use crate::ray_tracer::{
    common::vec3::structs::{Point3, Vec3},
    material::{classification::NullMaterial, traits::Material},
    render::ray::structs::Ray,
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p.clone(),
            normal: self.normal.clone(),
            t: self.t.clone(),
            front_face: self.front_face.clone(),
            material: self.material.clone(),
        }
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Arc::new(NullMaterial::new()),
        }
    }

    pub fn new_with_params(
        p: Point3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal.clone()) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
