use crate::tools::clamp;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;

pub type Color = Vec3;

pub fn out_color(color: Color, samples: i32) -> Color {
    const MAXC: f64 = 256.0;
    let mut r: f64 = color.x();
    let mut g: f64 = color.y();
    let mut b: f64 = color.z();

    let scale: f64 = 1.0 / samples as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    r = MAXC * clamp(r, 0.0, 0.999);
    g = MAXC * clamp(g, 0.0, 0.999);
    b = MAXC * clamp(b, 0.0, 0.999);

    Color::new(r, g, b)
}

pub fn write_color(file: &mut File, color: Color, samples: i32) {
    const MAXC: f64 = 256.0;
    let mut r: f64 = color.x();
    let mut g: f64 = color.y();
    let mut b: f64 = color.z();

    let scale: f64 = 1.0 / samples as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    file.write_all(
        format!(
            "{} {} {}\n",
            (MAXC * clamp(r, 0.0, 0.999)) as i32,
            (MAXC * clamp(g, 0.0, 0.999)) as i32,
            (MAXC * clamp(b, 0.0, 0.999)) as i32
        )
        .as_bytes(),
    )
    .unwrap();
}
