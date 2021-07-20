use std::{fmt::Debug, sync::Arc};

use crate::color::Color;
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
