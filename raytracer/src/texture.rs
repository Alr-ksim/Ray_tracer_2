use std::{fmt::Debug, sync::Arc};

use crate::color::Color;
use crate::perlin::Perlin;
use crate::vec3::Vec3;

pub trait Texture: Debug {
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
