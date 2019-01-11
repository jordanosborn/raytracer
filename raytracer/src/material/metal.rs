use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vec3;

use super::Scatter;

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::unit_vector(&r_in.direction()).reflect(&rec.normal);
        *scattered = Ray::new(&rec.p, &(reflected + self.fuzz * Vec3::random_in_unit_sphere()));
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
