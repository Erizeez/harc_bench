use crate::ray_tracer::{
    common::{extra::INFINITY, vec3::structs::Color},
    render::hit::hittable_list::HittableList,
};

use super::structs::Ray;

pub fn ray_color(r: Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
    }

    let (hit, rec) = world.hit(r.clone(), 0.001, INFINITY);

    if hit {
        let rec = rec.unwrap();
        let (res, attenuation, scattered) = rec.material.scatter(r.clone(), rec.clone());

        if res {
            return attenuation.unwrap() * ray_color(scattered.unwrap(), world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
