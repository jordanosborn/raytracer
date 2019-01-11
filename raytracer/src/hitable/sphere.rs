use super::Ray;
use super::Vec3;
use super::{HitRecord, Hitable};
use crate::material::MATERIAL;

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: MATERIAL,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: MATERIAL) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = 2.0 * Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let temp2 = (-b + discriminant.sqrt()) / (2.0 * a);
            match (temp1, temp2) {
                (t, _) | (_, t) if t_min < t && t < t_max => {
                    rec.t = t;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center) / self.radius;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}
