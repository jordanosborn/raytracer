use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vec3;

use super::Scatter;

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &(target - rec.p));
        *attenuation = self.albedo;
        true
    }
}
