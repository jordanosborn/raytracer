#![feature(test)]
extern crate test;
mod args;
mod camera;
mod hitable;
mod material;
mod ray;
mod vector;
use itertools::Itertools;
use clap::{Arg, App};
use indicatif::ParallelProgressIterator;
use self::camera::Camera;
use self::hitable::{
    hitable_list::{HitableList, HITABLE},
    sphere::Sphere,
    cube::Cube,
    HitRecord, Hitable,
};
use self::material::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Scatter, MATERIAL,
};
use self::ray::Ray;
use self::vector::Vec3;
use image::ImageBuffer;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

/**
 * Generates a random scene of n-4 tiny spheres, 1 large that all others lie on and 3 smaller ones
 */
fn random_scene(n: usize) -> HitableList {
    let mut list: Vec<HITABLE> = Vec::with_capacity(n + 1);
    let size = (f64::from(n as i32 - 4).sqrt() / 2.0) as i32;
    list.push(HITABLE::SPHERE(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));
    // list.push(HITABLE::CUBE(Cube::new(
    //     Vec3::new(200.0, -1000.0, 0.0),
    //     200.0,
    //     200.0,
    //     200.0,
    //     MATERIAL::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
    // )));
    let mut rng = rand::thread_rng();
    for a in -size..size {
        for b in -size..size {
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
        let mut scattered = Ray::new(&Vec3::zeros(), &Vec3::zeros());
        let mut attenuation = Vec3::zeros();
        //Loop and parallelise this?
        match rec.material {
            MATERIAL::Metal(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::zeros()
                }
            }
            MATERIAL::Lambertian(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::zeros()
                }
            }
            MATERIAL::Dielectric(mat) => {
                if depth < 50 && mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                    attenuation * color(&scattered, world, depth + 1u32)
                } else {
                    Vec3::zeros()
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
        (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let matches = App::new("raytracer")
        .version("0.0.1")
        .author("Jordan Osborn <jordan@osborn.dev>")
        // .about("Does awesome things")
        .arg(Arg::new("pixels_x")
            .short('x')
            .long("pixels-x")
            .value_name("PIXELS-X")
            .about("Sets width in pixels")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("pixels_y")
            .short('y')
            .long("pixels-y")
            .value_name("PIXELS-Y")
            .about("Sets height in pixels")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("samples")
            .short('s')
            .long("samples")
            .value_name("SAMPLES")
            .about("Sets number of samples")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::new("output_file")
            .short('o')
            .long("output")
            .value_name("OUTPUT")
            .about("Sets output file path")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let nx_i = matches.value_of("pixels_x").unwrap().to_string().parse::<u32>().unwrap();
    let ny_i = matches.value_of("pixels_y").unwrap().to_string().parse::<u32>().unwrap();
    let ns_i = matches.value_of("samples").unwrap().to_string().parse::<u32>().unwrap();
    let output = matches.value_of("output_file").unwrap();

    let (nx, ny, ns) = (f64::from(nx_i), f64::from(ny_i), f64::from(ns_i));

    let gamma = 2.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::zeros();
    
    let aperture = 0.1;
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;

    let dist_to_focus = (look_from - look_at).length();
    let aspect = nx / ny;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        fov,
        aspect,
        aperture,
        dist_to_focus,
    );

    let world = random_scene(500);

    let pb = ProgressBar::new(u64::from(ny_i * nx_i));
    pb.set_style(
        ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [{eta_precise}]")
    );


    let buffer_values = (0..(ny_i)).cartesian_product( (0..nx_i)).collect::<Vec<(u32, u32)>>().par_iter().progress_with(pb).map(|(j, i)| {
        let mut rng = rand::thread_rng();
        let random_numbers = (0..ns_i)
            .map(|_| (rng.gen::<f64>(), rng.gen::<f64>()))
            .collect::<Vec<(f64, f64)>>();
        let mut col: Vec3 = random_numbers.par_iter()
            .map(|(rand1, rand2)| {
                let u = (f64::from(*i) + rand1) / nx;
                let v = (f64::from(*j) as f64 + rand2) / ny;
                let r = camera.get_ray(u, v);
                color(&r, &world, 0u32)
            })
            .sum();
        col /= ns;
        //gamma correction
        col = col.apply(|&x| f64::powf(x, 1.0 / gamma));
        let ir = (255.99 * col[0]) as u8;
        let ig = (255.99 * col[1]) as u8;
        let ib = (255.99 * col[2]) as u8;
        (*i as u32, (ny_i - j - 1) as u32, image::Rgba([ir, ig, ib, 0xFF]))
    }).collect::<Vec<(u32, u32, image::Rgba<u8>)>>();

    let mut buffer = ImageBuffer::new(nx_i, ny_i);
    for (x, y, rgba) in buffer_values {
        buffer.put_pixel(x, y, rgba);
    }
    println!("Writing file to disk");
    buffer.save(&output).expect("File not saved");
    println!("{} saved", output);
}
