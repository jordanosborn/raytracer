use crate::vector::Vec3;
use super::Scatter;
use crate::hitable::HitRecord;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct Dielectric {
    refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {refractive_index}
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = (r_in.direction()).reflect(&rec.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::zeros();

        let (outward_normal, ni_over_nt) = if Vec3::dot(&r_in.direction(), &rec.normal) > 0.0  {
            (-rec.normal, self.refractive_index)
        } else {
            (rec.normal, 1.0 / self.refractive_index)
        };

        if (r_in.direction()).refract(&outward_normal, ni_over_nt, &mut refracted) {
            *scattered = Ray::new(&rec.p, &refracted);
            true

        } else {
            *scattered = Ray::new(&rec.p, &reflected);
            false
        }
    }
}