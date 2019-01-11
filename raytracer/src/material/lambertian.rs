use crate::vector::Vec3;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::hitable::HitRecord;

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {albedo}
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &(target - rec.p));
        *attenuation = self.albedo;
        true
    }
}
