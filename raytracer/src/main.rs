mod args;
mod hitable;
mod ray;
mod vector;
use self::hitable::{
    hitable_list::{HitableList, HITABLE},
    sphere::Sphere,
    HitRecord, Hitable,
};
use self::ray::Ray;
use self::vector::Vec3;
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
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output)
        .unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = HitableList::new(vec![
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    if let Err(e) = output.write(format!("P3\n{} {}\n255\n", nx_i, ny_i).as_bytes()) {
        panic!("Oh no {}", e)
    }

    for j in (0..(ny_i)).rev() {
        for i in 0..nx_i {
            let u = (i as f64) / nx;
            let v = (j as f64) / ny;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical).clone(),
            );
            let col = color(&r, &world);
            let ir = (255.99 * col[0]) as i64;
            let ig = (255.99 * col[1]) as i64;
            let ib = (255.99 * col[2]) as i64;
            if let Err(e) = output.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
                panic!("Oh no {}", e)
            }
        }
    }
}
