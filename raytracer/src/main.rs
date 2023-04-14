mod vec3;

use vec3::Vec3;

fn write_ppm(w: u32, h: u32, max_pixel_value: u32) { //simple ppm writer for testing
    println!("P3\n{} {}\n{}", w, h, max_pixel_value);

    for i in (0..h).rev() {
        for j in (0..w) {
            let r = j as f64 / (w - 1) as f64;
            let g = i as f64 / (h - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);

            }
    }
}


fn main() { //main function
    let width: u32 = 200;
    let height: u32 = 100;
    let max_pixel_value: u32 = 255;

    write_ppm(width, height, max_pixel_value);

    let v = Vec3::new();
}