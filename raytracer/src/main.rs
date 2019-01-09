mod args;
mod vector;
use self::vector::Vec3;

fn main() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let (width, height) = args::parse_args();
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
