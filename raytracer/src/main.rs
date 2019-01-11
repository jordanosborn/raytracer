mod args;
mod camera;
mod hitable;
mod material;
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
use image::ImageBuffer;
use indicatif::ProgressBar;
use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let ones = Vec3::new(1.0, 1.0, 1.0);
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - ones;
        if p.squared_length() < 1.0 {
            break;
        }
    }
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

    let camera = Camera::new();
    let gamma = 2.0;

    let mut buffer = ImageBuffer::new(nx_i as u32, ny_i as u32);

    let world = HitableList::new(vec![
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        HITABLE::SPHERE(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

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
            col = col.apply(|&x| f64::powf(x, 1.0 / gamma));
            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;
            buffer.put_pixel(
                i as u32,
                (ny_i - j - 1) as u32,
                image::Rgba([ir, ig, ib, 0xFF]),
            );
        }
        pb.inc(1);
    }
    pb.finish();
    buffer.save(output).expect("File not saved");
}
