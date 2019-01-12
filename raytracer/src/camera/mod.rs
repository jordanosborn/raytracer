use super::ray::Ray;
use super::vector::Vec3;
use std::f64::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vertical_fov: f64,
        aspect: f64,
    ) -> Camera {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        let v = Vec3::cross(&w, &u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin),
        )
    }
}
