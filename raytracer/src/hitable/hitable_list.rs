use super::Ray;
use super::{sphere::Sphere, HitRecord, Hitable};

#[derive(Copy, Clone)]
pub enum HITABLE {
    SPHERE(Sphere),
}

#[derive(Clone)]
pub struct HitableList {
    length: usize,
    list: Vec<HITABLE>,
}

impl HitableList {
    pub fn new(list: Vec<HITABLE>) -> HitableList {
        let length = list.len();
        HitableList { list, length }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.length {
            match self.list[i] {
                HITABLE::SPHERE(v) => {
                    if v.hit(r, t_min, closest_so_far, &mut temp_rec) {
                        hit_anything = true;
                        closest_so_far = temp_rec.t;
                        rec.normal = temp_rec.normal;
                        rec.p = temp_rec.p;
                        rec.t = temp_rec.t;
                        rec.material = v.material;
                    }
                }
            }
        }
        hit_anything
    }
}

impl std::ops::Add for HitableList {
    type Output = HitableList;
    fn add(self, hl2: HitableList) -> HitableList {
        let mut new_list = self.list.clone();
        let mut old_list = hl2.clone();
        new_list.append(&mut old_list.list);
        HitableList {
            length: self.length + hl2.length,
            list: new_list,
        }
    }
}
