mod vec3;
mod ray;

use ray::Ray;
use vec3::Vec3;

// fn write_ppm(w: u32, h: u32, max_pixel_value: u32) { //simple ppm writer for testing
// }
fn color(r: &Ray) -> Vec3 {
    let unitDirection = Vec3::unitVector(&r.direction());
    let t: f32 = 0.5 * (unitDirection.y() + 1.0);   

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}


fn main() { //main function
    let w: u32 = 200;
    let h: u32 = 100;
    let max_pixel_value: u32 = 255;

    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);


    println!("P3\n{} {}\n{}", w, h, max_pixel_value);

    for i in (0..h).rev() {
        for j in 0..w {
            let u: f32 = j as f32 / (w - 1) as f32;
            let v: f32 = i as f32 / (h - 1) as f32;
            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);    
            let col = color(&r);
            // let b: f64 = 0.25;

            let ir = (255.999 * col.r()) as u32;
            let ig = (255.999 * col.g()) as u32;
            let ib = (255.999 * col.b()) as u32;

            println!("{} {} {}", ir, ig, ib);

            }
    }
}