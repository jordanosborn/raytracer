mod args;
mod camera;
mod hitable;
mod ray;
mod vector;
use self::camera::Camera;
use self::hitable::{
    hitable_list::{HitableList, HITABLE},
    sphere::Sphere,
    HitRecord, Hitable,
};
use self::ray::Ray;
use self::vector::Vec3;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, std::f64::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (nx_i, ny_i, output) = args::parse_args();
    let (nx, ny) = (nx_i as f64, ny_i as f64);
    let ns = 100;
    let mut rng = rand::thread_rng();
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output)
        .unwrap();

    let camera = Camera::new();

    let world = HitableList::new(vec![
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    if let Err(e) = output.write(format!("P3\n{} {}\n255\n", nx_i, ny_i).as_bytes()) {
        panic!("Oh no {}", e)
    }

    for j in (0..(ny_i)).rev() {
        for i in 0..nx_i {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let (rand1, rand2): (f64, f64) = (rng.gen(), rng.gen());
                let u: f64 = (i as f64 + rand1) / nx;
                let v: f64 = (j as f64 + rand2) / ny;
                let r = camera.get_ray(u, v);
                col += color(&r, &world);
            }

            col /= ns as f64;
            let ir = (255.99 * col[0]) as i64;
            let ig = (255.99 * col[1]) as i64;
            let ib = (255.99 * col[2]) as i64;
            if let Err(e) = output.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
                panic!("Oh no {}", e)
            }
        }
    }
}
