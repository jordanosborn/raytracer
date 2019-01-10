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
use indicatif::ProgressBar;

fn random_in_unit_sphere() -> Vec3{
    let mut rng = rand::thread_rng();
    let ones = Vec3::new(1.0, 1.0, 1.0);
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - ones;
        if p.squared_length() < 1.0 {
            break;
        }
    };
    p
}

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    let mut rec = HitRecord::new();
    // ignores hits very close to zero
    if world.hit(ray, 0.001, std::f64::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(&rec.p, &(target - rec.p)), &world)
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (nx_i, ny_i, ns, output) = args::parse_args();
    let (nx, ny) = (nx_i as f64, ny_i as f64);
    let mut rng = rand::thread_rng();
    let mut output = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output)
        .unwrap();

    let camera = Camera::new();
    let gamma = 2.0;

    let world = HitableList::new(vec![
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    if let Err(e) = output.write(format!("P3\n{} {}\n255\n", nx_i, ny_i).as_bytes()) {
        panic!("Oh no {}", e)
    }
    let pb = ProgressBar::new((ny_i) as u64);
    for j in (0..(ny_i)).rev() {
        // write to file in batches (pixel rows)
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
            //gamma correction
            col = col.apply(|&x| {
                f64::powf(x, 1.0 / gamma)
            });
            let ir = (255.99 * col[0]) as i64;
            let ig = (255.99 * col[1]) as i64;
            let ib = (255.99 * col[2]) as i64;
            if let Err(e) = output.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
                panic!("Oh no {}", e)
            }
            
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done");
}
