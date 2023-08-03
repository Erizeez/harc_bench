use std::sync::Arc;

use crate::ray_tracer::render::ray::structs::Ray;

use super::{hit_record::HitRecord, traits::Hittable};

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let (hit, rec) = object.hit(r.clone(), t_min, closest_so_far);
            if hit {
                hit_anything = true;
                temp_rec = rec.unwrap();
                closest_so_far = temp_rec.t;
            }
        }

        if hit_anything {
            (hit_anything, Some(temp_rec))
        } else {
            (hit_anything, None)
        }
    }
}
