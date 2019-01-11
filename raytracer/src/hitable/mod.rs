pub mod hitable_list;
pub mod sphere;
use super::material::{MATERIAL, lambertian::Lambertian};
use super::ray::Ray;
use super::vector::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<MATERIAL>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::new(MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.0, 0.0, 0.0)))),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
