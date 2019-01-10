mod args;
mod vector;
use self::vector::Vec3;

fn main() {
    let (width_i, height_i) = args::parse_args();
    let (width, height) = (width_i as f64, height_i as f64);
    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..(height_i)).rev() {
        for i in 0..width_i {
            let col = Vec3::new((i as f64) / width, (j as f64) / height, 0.2);
            let ir = (255.99 * col[0]) as i64;
            let ig = (255.99 * col[1]) as i64;
            let ib = (255.99 * col[2]) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
