use std::fs::File;
use std::io::BufWriter;

mod vec3;
mod ray;
mod hit;
mod sphere;
mod material;
mod ppm;

pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::Hit;
pub use sphere::Sphere;
pub use material::{Material, Texture};
pub use ppm::write_color;
pub use std::io::Write;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0 - Vec3::new(1.0, 1.0, 1.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
