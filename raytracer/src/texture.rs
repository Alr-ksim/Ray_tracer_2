extern crate image;
use core::f64;
use std::{fmt::Debug, sync::Arc};

use crate::color::Color;
use crate::perlin::Perlin;
use crate::tools;
use crate::vec3::Vec3;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
    pub fn color(&self) -> Vec3 {
        self.color
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color()
    }
}

#[derive(Debug, Clone)]
pub struct CheckerTexture {
    odd: Arc<Texture>,
    even: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<Texture>, even: Arc<Texture>) -> Self {
        Self { odd, even }
    }
    pub fn cnew(c1: Color, c2: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sp = (*p).clone() * self.scale;
        let sz = p.z() * self.scale;
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (sz + 10.0 * self.noise.turb(&sp, 7)).sin())
    }
}

const BYTES_PER_PIXEL: usize = 3;
use image::GenericImage;
use imageproc::drawing::Canvas;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn emnew() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }
    pub fn new(path: &Path) -> Self {
        let cp_pixel = BYTES_PER_PIXEL;

        let img = image::open(path).unwrap();

        let bps = img.width() as usize * BYTES_PER_PIXEL;

        Self {
            data: img.to_bytes(),
            width: img.width() as usize,
            height: img.height() as usize,
            bytes_per_scanline: bps,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let uu = tools::clamp(u, 0.0, 1.0);
        let vv = 1.0 - tools::clamp(v, 0.0, 1.0);

        let mut i = (uu * self.width as f64) as usize;
        let mut j = (vv * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        const CSCALE: f64 = 1.0 / 255.0;
        let pixel = j * self.bytes_per_scanline + i * BYTES_PER_PIXEL;

        Color::new(
            CSCALE * self.data[pixel] as f64,
            CSCALE * self.data[pixel + 1] as f64,
            CSCALE * self.data[pixel + 2] as f64,
        )
    }
}
