use super::vector::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new() -> Ray {
        Ray {a: Vec3::new(0.0, 0.0, 0.0), b: Vec3::new(0.0,0.0,0.0)}
    }
    pub fn new_from(a: &Vec3, b: &Vec3) -> Ray {
        Ray {a: *a, b: *b}
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}