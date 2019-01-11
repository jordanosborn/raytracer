use crate::vector::Vec3;
use super::Scatter;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use rand::Rng;

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

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
        let mut rng = rand::thread_rng();
        let reflected = (r_in.direction()).reflect(&rec.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::zeros();
        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(&r_in.direction(), &rec.normal) > 0.0  {
            (-rec.normal, self.refractive_index, self.refractive_index * Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length())
        } else {
            (rec.normal, 1.0 / self.refractive_index, -Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length())
        };

        let reflect_prob = if (r_in.direction()).refract(&outward_normal, ni_over_nt, &mut refracted) {
            schlick(cosine, self.refractive_index)
        } else {
            1.0
        };

        if rng.gen::<f64>() < reflect_prob {
            *scattered = Ray::new(&rec.p, &reflected);
        } else {
            *scattered = Ray::new(&rec.p, &refracted);
        }
        true
    }
}