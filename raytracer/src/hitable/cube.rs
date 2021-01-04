use super::Ray;
use super::Vec3;
use super::{HitRecord, Hitable};
use crate::material::MATERIAL;

#[derive(Copy, Clone)]
pub struct Cube {
    corner1: Vec3,
    corner2: Vec3,
    pub material: MATERIAL,
}

impl Cube {
    pub fn new(bottom_corner: Vec3, width: f64, height: f64, depth: f64, material: MATERIAL) -> Cube {
        Cube {
            corner1: bottom_corner,
            corner2: Vec3::new(bottom_corner.x() + width, bottom_corner.y() + height, bottom_corner.z() + depth),
            material: material,
        }
    }

    pub fn width(&self) -> f64 {
        self.corner2.x() - self.corner1.x()
    }

    pub fn height(&self) -> f64 {
        self.corner2.y() - self.corner1.y()
    }

    pub fn depth(&self) -> f64 {
        self.corner2.z() - self.corner1.z()
    }

}

impl Hitable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
        // let oc = r.origin() - self.center;
        // let a = Vec3::dot(&r.direction(), &r.direction());
        // let b = 2.0 * Vec3::dot(&oc, &r.direction());
        // let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        // let discriminant = b * b - 4.0 * a * c;
        // if discriminant > 0.0 {
        //     let temp1 = (-b - discriminant.sqrt()) / (2.0 * a);
        //     let temp2 = (-b + discriminant.sqrt()) / (2.0 * a);
        //     match (temp1, temp2) {
        //         (temp, _) | (_, temp) if t_min < temp && temp < t_max => {
        //             rec.t = temp;
        //             rec.p = r.point_at_parameter(rec.t);
        //             rec.normal = (rec.p - self.center) / self.radius;
        //             true
        //         }
        //         _ => false,
        //     }
        // } else {
        //     false
        // }
    }
}
