use std::io::prelude::*;

use crate::vec3::Vec3;

pub fn write_color(mut file: impl Write, color: Vec3, samples_per_pixel: u32) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * color.e[0]).sqrt();
    let g = (scale * color.e[1]).sqrt();
    let b = (scale * color.e[2]).sqrt();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

    writeln!(file, "{} {} {}", ir, ig, ib)
}