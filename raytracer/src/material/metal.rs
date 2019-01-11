use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vec3;

use super::Scatter;

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
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
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
