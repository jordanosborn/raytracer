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
use self::material::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Scatter, MATERIAL,
};
use self::ray::Ray;
use self::vector::Vec3;
use image::ImageBuffer;
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

fn random_scene() -> HitableList {
    let n = 500;
    let mut list: Vec<HITABLE> = Vec::with_capacity(n + 1);
    list.push(HITABLE::SPHERE(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            //diffuse
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(HITABLE::SPHERE(Sphere::new(
                        center,
                        0.2,
                        MATERIAL::Lambertian(Lambertian::new(Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    list.push(HITABLE::SPHERE(Sphere::new(
                        center,
                        0.2,
                        MATERIAL::Metal(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    )));
                } else {
                    //glass
                    list.push(HITABLE::SPHERE(Sphere::new(
                        center,
                        0.2,
                        MATERIAL::Dielectric(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    list.push(HITABLE::SPHERE(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        MATERIAL::Dielectric(Dielectric::new(1.5)),
    )));
    list.push(HITABLE::SPHERE(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.push(HITABLE::SPHERE(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        MATERIAL::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    HitableList::new(list)
}

fn color(ray: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    // ignores hits very close to zero
    if world.hit(ray, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        match rec.material {
            MATERIAL::Metal(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
            MATERIAL::Lambertian(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
            MATERIAL::Dielectric(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
            MATERIAL::Empty => {
                let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
                0.5 * color(&Ray::new(&rec.p, &(target - rec.p)), &world, 0u32)
            }
        }
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (nx_i, ny_i, ns, output) = args::parse_args();
    let (nx, ny) = (f64::from(nx_i), f64::from(ny_i));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx / ny,
    );
    let gamma = 2.0;

    let mut buffer = ImageBuffer::new(nx_i, ny_i);
    let world = random_scene();
    // let world = HitableList::new(vec![
    //     HITABLE::SPHERE(Sphere::new(
    //         Vec3::new(0.0, 0.0, -1.0),
    //         0.5,
    //         MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    //     )),
    //     HITABLE::SPHERE(Sphere::new(
    //         Vec3::new(0.0, -100.5, -1.0),
    //         100.0,
    //         MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.0))),
    //     )),
    //     HITABLE::SPHERE(Sphere::new(
    //         Vec3::new(1.0, 0.0, -1.0),
    //         0.5,
    //         MATERIAL::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    //     )),
    //     HITABLE::SPHERE(Sphere::new(
    //         Vec3::new(-1.0, 0.0, -1.0),
    //         -0.45,
    //         MATERIAL::Dielectric(Dielectric::new(1.5)),
    //     )),
    //     HITABLE::SPHERE(Sphere::new(
    //         Vec3::new(-1.0, 0.0, -1.0),
    //         0.5,
    //         MATERIAL::Dielectric(Dielectric::new(1.5)),
    //     )),
    // ]);

    let pb = ProgressBar::new(u64::from(ny_i));

    (0..(ny_i)).rev().for_each(|j| {
        let row: Vec<(u32, u32, [u8; 4])> = (0..nx_i)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();
                let random_numbers = (0..ns)
                    .map(|_| (rng.gen::<f64>(), rng.gen::<f64>()))
                    .collect::<Vec<(f64, f64)>>();
                let mut col: Vec3 = random_numbers
                    .par_iter()
                    .map(|(rand1, rand2)| {
                        let u = (f64::from(i) + rand1) / nx;
                        let v = (f64::from(j) as f64 + rand2) / ny;
                        let r = camera.get_ray(u, v);
                        color(&r, &world, 0u32)
                    })
                    .sum();
                col /= f64::from(ns);
                //gamma correction
                col = col.apply(|&x| f64::powf(x, 1.0 / gamma));
                let ir = (255.99 * col[0]) as u8;
                let ig = (255.99 * col[1]) as u8;
                let ib = (255.99 * col[2]) as u8;
                (i as u32, (ny_i - j - 1) as u32, [ir, ig, ib, 0xFF])
            })
            .collect();
        row.iter()
            .for_each(|(x, y, rgba)| buffer.put_pixel(*x, *y, image::Rgba(*rgba)));

        pb.inc(1);
    });
    pb.finish();
    println!("Writing file to disk");
    buffer.save(&output).expect("File not saved");
    println!("{} saved", output);
}
