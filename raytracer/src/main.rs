use std::fs::File;
use std::io::BufWriter;

mod vec3;
mod ray;
mod hit;
mod sphere;
mod material;
mod ppm;

use vec3::Vec3;
use ray::Ray;
use hit::Hit;
use sphere::Sphere;
use material::{Material, Texture};
use ppm::write_color;

fn main() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world: Vec<Sphere> = vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::new(Texture::SolidColor(Vec3::new(0.7, 0.3, 0.3))),
        ),
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::new(Texture::SolidColor(Vec3::new(0.8, 0.8, 0.0))),
        ),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let file = File::create("output.ppm")?;
    let mut buf_writer = BufWriter::new(file);
    writeln!(buf_writer, "P3")?;
    writeln!(buf_writer, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(buf_writer, "255")?;

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
                color += ray_color(ray, &world, 0);
            }
            write_color(&mut buf_writer, color, SAMPLES_PER_PIXEL)?;
        }
    }

    buf_writer.flush()?;

    Ok(())
}

fn ray_color(ray: Ray, world: &[Sphere], depth: u32) -> Vec3 {
    if depth >= 50 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = hit_world(ray, world, 0.001, f64::INFINITY) {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        let attenuation = hit.material.color(0.0, 0.0, hit.point);
        return attenuation * ray_color(Ray::new(hit.point, target - hit.point), world, depth + 1);
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_world(ray: Ray, world: &[Sphere], t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest_so_far = t_max;
    let mut hit = None;

    for sphere in world {
        if let Some(temp_hit) = sphere.hit(ray, t_min, closest_so_far) {
            closest_so_far = temp_hit.t;
            hit = Some(temp_hit);
        }
    }

    hit
}

fn random_in_unit_sphere() -> Vec3 {
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
