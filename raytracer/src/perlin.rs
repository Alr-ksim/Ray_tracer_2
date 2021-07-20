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
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
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

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let di = i as f64;
                    let dj = j as f64;
                    let dk = k as f64;
                    accum += (di * u + (1.0 - di) * (1.0 - u))
                        * (dj * v + (1.0 - dj) * (1.0 - v))
                        * (dk * w + (1.0 - dk) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}
