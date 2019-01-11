pub mod lambertian;
use self::lambertian::Lambertian;

#[derive(Clone, Copy)]
pub enum MATERIAL {
    Lambertian(Lambertian)
}


