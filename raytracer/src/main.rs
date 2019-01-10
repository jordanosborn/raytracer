mod args;
mod vector;
mod ray;
use self::vector::Vec3;
use self::ray::Ray;


fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> (bool, f64) {
    let oc = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        (false, -1.0)
    } else {
        // closest solution only
        (true, (-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn color(ray: &Ray) -> Vec3 {
    let (hit, t) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if hit {
        let n = Vec3::unit_vector(&(ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = Vec3::unit_vector(&ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let (width_i, height_i) = args::parse_args();
    let (width, height) = (width_i as f64, height_i as f64);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    print!("P3\n{} {}\n255\n", width, height);

    for j in (0..(height_i)).rev() {
        for i in 0..width_i {
            let u = (i as f64) / width;
            let v = (j as f64) / height;
            let r = Ray::new_from(&origin, &(lower_left_corner + u * horizontal + v * vertical));
            let col = color(&r);
            let ir = (255.99 * col[0]) as i64;
            let ig = (255.99 * col[1]) as i64;
            let ib = (255.99 * col[2]) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
