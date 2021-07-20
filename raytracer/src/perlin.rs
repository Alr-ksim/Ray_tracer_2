use crate::tools;
use crate::vec3::Vec3;
use std::vec::Vec;
use std::{fmt::Debug, sync::Arc};

const PCNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rv: Vec<Vec3> = Vec::new();
        for i in 0..PCNT {
            rv.push(Vec3::randvr(-1.0, 1.0));
        }
        let mut px: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut px);
        let mut py: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut py);
        let mut pz: Vec<usize> = Vec::new();
        Perlin::perlin_generate_perm(&mut pz);
        Self {
            ranvec: rv,
            perm_x: px,
            perm_y: py,
            perm_z: pz,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }
        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut tem = *p;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&tem);
            weight *= 0.5;
            tem *= 2.0;
        }

        accum.abs()
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

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let di = i as f64;
                    let dj = j as f64;
                    let dk = k as f64;
                    let weight_v = Vec3::new(u - di, v - dj, w - dk);
                    accum += (di * uu + (1.0 - di) * (1.0 - uu))
                        * (dj * vv + (1.0 - dj) * (1.0 - vv))
                        * (dk * ww + (1.0 - dk) * (1.0 - ww))
                        * (c[i][j][k].clone() * weight_v);
                }
            }
        }
        accum
    }
}
