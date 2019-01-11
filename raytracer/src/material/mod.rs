pub mod lambertian;
pub mod metal;
use self::lambertian::Lambertian;
use self::metal::Metal;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub enum MATERIAL {
    Lambertian(Lambertian),
    Metal(Metal),
    Empty
}

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
