use crate::ray_tracer::common::vec3::structs::Vec3;

pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(
            rand::random::<f64>() * 2.0 - 1.0,
            rand::random::<f64>() * 2.0 - 1.0,
            0.0,
        );
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}
