use self::util::random_in_unit_disk;

use super::{
    common::vec3::structs::{Point3, Vec3},
    render::ray::structs::Ray,
};

mod util;

pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom.clone() - lookat.clone()).unit_vector();
        let u = vup.clone().cross(w.clone()).unit_vector();
        let v = w.clone().cross(u.clone());

        let origin = lookfrom.clone();
        let horizontal = focus_dist * viewport_width * u.clone();
        let vertical = focus_dist * viewport_height * v.clone();
        let lower_left_corner = origin.clone()
            - horizontal.clone() / 2.0
            - vertical.clone() / 2.0
            - focus_dist * w.clone();

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();

        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone()
                + s * self.horizontal.clone()
                + t * self.vertical.clone()
                - self.origin.clone()
                - offset.clone(),
        )
    }
}
