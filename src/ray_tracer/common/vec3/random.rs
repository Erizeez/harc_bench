use rand::Rng;

use super::structs::Vec3;

pub fn random() -> Vec3 {
    Vec3 {
        x: rand::random::<f64>(),
        y: rand::random::<f64>(),
        z: rand::random::<f64>(),
    }
}

pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3 {
        x: rand::thread_rng().gen_range(min..max),
        y: rand::thread_rng().gen_range(min..max),
        z: rand::thread_rng().gen_range(min..max),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_hemi_sphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}
