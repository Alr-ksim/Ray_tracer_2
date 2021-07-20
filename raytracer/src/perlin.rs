use crate::tools;
use crate::vec3::Vec3;
use std::vec::Vec;
use std::{fmt::Debug, sync::Arc};

const PCNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rf: Vec<f64> = Vec::new();
        for i in 0..PCNT {
            rf.push(tools::randf(0.0, 1.0));
        }
        let mut px: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut px);
        let mut py: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut py);
        let mut pz: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut pz);
        Self {
            ranfloat: rf,
            perm_x: px,
            perm_y: py,
            perm_z: pz,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = (255 & (4.0 * p.x()) as i32) as usize;
        let j = (255 & (4.0 * p.y()) as i32) as usize;
        let k = (255 & (4.0 * p.z()) as i32) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(v: &mut Vec<usize>) {
        for i in 0..PCNT {
            v.push(i);
        }
        Perlin::permute(v, PCNT);
    }

    fn permute(v: &mut Vec<usize>, n: usize) {
        for i in 0..(PCNT - 1) {
            let tar = tools::randi((i + 1) as i32, (PCNT - 1) as i32) as usize;
            let tmp = v[i];
            v[i] = v[tar];
            v[tar] = tmp;
        }
    }
}
