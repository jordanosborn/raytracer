use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    ///Vector class
    pub data: [f64; 3],
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.data[0], self.data[1], self.data[2])
    }
}
impl std::iter::Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        let mut s = Vec3::new(0.0, 0.0, 0.0);
        for x in iter {
            s += x;
        }
        s
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }
    pub fn new_from(v: &Vec3) -> Vec3 {
        Vec3 {
            data: [v.data[0], v.data[1], v.data[2]],
        }
    }

    pub fn zeros() -> Vec3 {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            data: [1.0, 1.0, 1.0],
        }
    }
    #[inline]
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    #[inline]
    pub fn r(&self) -> f64 {
        self.data[0]
    }
    #[inline]
    pub fn g(&self) -> f64 {
        self.data[1]
    }
    #[inline]
    pub fn b(&self) -> f64 {
        self.data[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2])
            .sqrt()
    }

    #[inline]
    pub fn squared_length(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }

    #[inline]
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let length = (v.data[0] * v.data[0] + v.data[1] * v.data[1] + v.data[2] * v.data[2]).sqrt();
        Vec3 {
            data: [v.data[0] / length, v.data[1] / length, v.data[2] / length],
        }
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        (*self) - 2.0 * Vec3::dot(self, normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
        let uv = Vec3::unit_vector(self);
        let dt = Vec3::dot(&uv, &normal);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - (*normal) * dt) - (*normal) * discriminant.sqrt();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        let length = (self.data[0] * self.data[0]
            + self.data[1] * self.data[1]
            + self.data[2] * self.data[2])
            .sqrt();
        self.data[0] /= length;
        self.data[1] /= length;
        self.data[2] /= length;
    }

    #[inline]
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.data[0] * v2.data[0] + v1.data[1] * v2.data[1] + v1.data[2] * v2.data[2]
    }

    #[inline]
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            data: [
                v1.data[1] * v2.data[2] - v1.data[2] * v2.data[1],
                v1.data[2] * v2.data[0] - v1.data[0] * v2.data[2],
                v1.data[0] * v2.data[1] - v1.data[1] * v2.data[0],
            ],
        }
    }

    pub fn apply<F>(&self, f: F) -> Vec3
    where
        F: Fn(&f64) -> f64,
    {
        Vec3 {
            data: [f(&self.data[0]), f(&self.data[1]), f(&self.data[2])],
        }
    }
    pub fn all<F>(&self, f: F) -> bool
    where
        F: Fn(&f64) -> bool,
    {
        f(&self.x()) && f(&self.y()) && f(&self.z())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let ones = Vec3::ones();
        let mut p: Vec3;
        loop {
            p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - ones;
            if p.squared_length() < 1.0 {
                break;
            }
        }
        p
    }
    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let ones = Vec3::new(1.0, 1.0, 0.0);
        let mut p: Vec3;
        loop {
            p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - ones;
            if p.squared_length() < 1.0 {
                break;
            }
        }
        p
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &f64 {
        if idx < 3 {
            &self.data[idx]
        } else {
            panic!("Index invalid")
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        if idx < 3 {
            &mut self.data[idx]
        } else {
            panic!("Index invalid")
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
            ],
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self * other.data[0],
                self * other.data[1],
                self * other.data[2],
            ],
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            data: [
                self.data[0] * other,
                self.data[1] * other,
                self.data[2] * other,
            ],
        }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.data[0] * other.data[0],
                self.data[1] * other.data[1],
                self.data[2] * other.data[2],
            ],
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, other: f64) -> Vec3 {
        if other != 0.0 {
            Vec3 {
                data: [
                    self.data[0] / other,
                    self.data[1] / other,
                    self.data[2] / other,
                ],
            }
        } else {
            panic!("Can't divide by zero")
        }
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        if other.data[0] != 0.0 && other.data[1] != 0.0 && other.data[2] != 0.0 {
            Vec3 {
                data: [
                    self.data[0] / other.data[0],
                    self.data[1] / other.data[1],
                    self.data[2] / other.data[2],
                ],
            }
        } else {
            panic!("Can't divide by zero")
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.data[0] = self.data[0] + other.data[0];
        self.data[1] = self.data[1] + other.data[1];
        self.data[2] = self.data[2] + other.data[2];
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.data[0] = self.data[0] - other.data[0];
        self.data[1] = self.data[1] - other.data[1];
        self.data[2] = self.data[2] - other.data[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.data[0] = self.data[0] * other;
        self.data[1] = self.data[1] * other;
        self.data[2] = self.data[2] * other;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.data[0] = self.data[0] * other.data[0];
        self.data[1] = self.data[1] * other.data[1];
        self.data[2] = self.data[2] * other.data[2];
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        if other != 0.0 {
            self.data[0] = self.data[0] / other;
            self.data[1] = self.data[1] / other;
            self.data[2] = self.data[2] / other;
        } else {
            panic!("Can't divide by zero")
        }
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        if other.data[0] != 0.0 && other.data[1] != 0.0 && other.data[2] != 0.0 {
            self.data[0] = self.data[0] / other.data[0];
            self.data[1] = self.data[1] / other.data[1];
            self.data[2] = self.data[2] / other.data[2];
        } else {
            panic!("Can't divide by zero")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let x = Vec3::new(1.0, 2.0, 3.0);
        let y = Vec3::new(1.0, 2.0, 4.0);
        let z = Vec3::new(2.0, 4.0, 7.0);
        assert!((x + y - z).all(|&x| x < std::f64::EPSILON));
    }
    #[test]
    fn test_sub() {
        let x = Vec3::new(1.0, 2.0, 3.0);
        let y = Vec3::new(1.0, 2.0, 4.0);
        let z = Vec3::new(0.0, 0.0, -1.0);
        assert!((x - y - z).all(|&x| x < std::f64::EPSILON));
    }

    #[test]
    fn test_select() {
        let x = Vec3::new(1.0, 2.0, 3.0);
        assert!((x[2] - x.z()).abs() < std::f64::EPSILON);
        assert!((x[2] - x.b()).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_random_in_unit_disk() {
        for _ in 0..100 {
            let x = Vec3::random_in_unit_disk();
            assert!(x.length() < 1.0);
            assert!(x[2] == 0.0);
        }
    }
    #[test]
    fn test_random_in_unit_sphere() {
        for _ in 0..100 {
            let x = Vec3::random_in_unit_sphere();
            assert!(x.length() < 1.0);
        }
    }
}
