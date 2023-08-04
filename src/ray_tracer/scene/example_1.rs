use std::sync::{Arc, RwLock};

use crate::ray_tracer::{
    common::{
        extra::{random_double, random_double_range},
        vec3::structs::{Color, Point3},
    },
    material::{
        classification::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
        traits::Material,
    },
    object::sphere::Sphere,
    render::hit::hittable_list::HittableList,
};

pub fn generate_example_scene_1() -> Arc<RwLock<HittableList>> {
    let world = Arc::new(RwLock::new(HittableList::new()));

    // random scene
    let material_ground = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center.clone() - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian { albedo });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric { ir: 1.5 });
                    world.write().unwrap().add(Arc::new(Sphere::new(
                        center.clone(),
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.write().unwrap().add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

    world
}
