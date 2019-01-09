fn parse_args() -> (i32, i32) {
    match std::env::args().collect::<Vec<String>>().as_slice() {
        [_, a, b] => {
            match (a.parse::<i32>(), b.parse::<i32>()) {
                (Ok(a), Ok(b)) => {(a, b)}
                _ => panic!("Incorrect command line args given")
            }
        }
        _ => panic!("Incorrect command line args given")
    }
}

struct Vec3 {
    pub data: [f64;3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {data: [x, y, z]}
    }
    #[inline] 
    pub fn x(&self) -> f64 {self.data[0]}
    #[inline] 
    pub fn y(&self) -> f64 {self.data[0]}
    #[inline] 
    pub fn z(&self) -> f64{self.data[0]}
    #[inline] 
    pub fn r(&self) -> f64 {self.data[0]}
    #[inline] 
    pub fn g(&self) -> f64 {self.data[0]}
    #[inline] 
    pub fn b(&self) -> f64 {self.data[0]}

    #[inline]
    pub fn length(&self) -> f64 {
        (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]).sqrt()
    }
    #[inline]
    pub fn squared_length(&self) -> f64 {
        self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]
    }
    #[inline]
    pub fn make_unit_vector(&self) -> Vec3 {
        let length = (self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2]).sqrt();
        Vec3{
            data: [self.data[0] / length, self.data[1] / length, self.data[2] / length]
        }
    }
}


impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3{data: [
            self.data[0] + other.data[0],
            self.data[1] + other.data[1],
            self.data[2] + other.data[2]
        ]}
    }
}

fn main() {
    let (width, height) = parse_args();
    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..(height)).rev() {
        for i in 0..width {
            let r = (i as f64) / (width as f64);
            let g = (j as f64) / (height as f64);
            let b = 0.2;
            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}