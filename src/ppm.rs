use std::io::prelude::*;

use crate::vec3::Vec3;

pub fn write_color(mut file: impl Write, color: Vec3, samples_per_pixel: u32) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = color.e[0];
    let g = color.e[1];
    let b = color.e[2];

    // Increase the brightness by adjusting the SCALE_FACTOR
    const SCALE_FACTOR: f64 = 1.2; // you can change this value to adjust the brightness

    let r = (r * scale * SCALE_FACTOR).sqrt();
    let g = (g * scale * SCALE_FACTOR).sqrt();
    let b = (b * scale * SCALE_FACTOR).sqrt();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

    writeln!(file, "{} {} {}", ir, ig, ib)
}
