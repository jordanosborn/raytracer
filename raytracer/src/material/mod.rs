pub mod lambertian;
pub mod metal;
use self::lambertian::Lambertian;
use self::metal::Metal;

#[derive(Clone, Copy)]
pub enum MATERIAL {
    Lambertian(Lambertian),
    Metal(Metal),
}
