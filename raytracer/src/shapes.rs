use crate::material::Material;
use crate::material::Neg;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fmt::Debug;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct Hitrec<'a> {
    pub p: Vec3,
    pub nf: Vec3,
    pub t: f64,
    pub front_face: bool, // true: hit outsides
    pub mat: &'a dyn Material,
}

impl<'a> Hitrec<'a> {
    pub fn new(nmat: &'a dyn Material) -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            nf: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: nmat,
        }
    }
    pub fn p(&self) -> Vec3 {
        self.p.clone()
    }
    pub fn nf(&self) -> Vec3 {
        self.nf.clone()
    }
    pub fn set_face(&mut self, r: Ray, nf: Vec3) {
        self.front_face = (r.diraction() * nf.clone() < 0.0);
        self.nf = if self.front_face {
            nf.clone()
        } else {
            -(nf.clone())
        };
    }
    pub fn copy(&mut self, rec: Self) {
        self.p = rec.p.clone();
        self.nf = rec.nf.clone();
        self.t = rec.t;
        self.front_face = rec.front_face;
        self.mat = rec.mat;
    }
}

#[derive(Debug)]
pub struct Sphere<M: Material> {
    pub ct: Vec3,
    pub rad: f64,
    pub mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(ct: Vec3, rad: f64, mat: M) -> Self {
        Self { ct, rad, mat }
    }
    pub fn ct(&self) -> Vec3 {
        self.ct.clone()
    }
}

#[derive(Debug)]
pub struct  MovingSphere<M: Material> {
    pub ct0: Vec3,
    pub ct1: Vec3,
    pub tm0: f64,
    pub tm1: f64,
    pub rad: f64,
    pub mat: M,
}

impl <M: Material> MovingSphere<M> {
    pub fn new(ct0: Vec3, ct1: Vec3, tm0: f64, tm1: f64, rad: f64, mat: M) -> Self {
        Self { ct0, ct1, tm0, tm1, rad, mat }
    }

    pub fn ct(&self, tm: f64) -> Vec3 {
        self.ct0.clone() + (self.ct1.clone() - self.ct0.clone())*((tm - self.tm0)/(self.tm1-self.tm0))
    }
}

pub trait Hittable: Debug {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec>;
}

#[derive(Debug)]
pub struct Hitlist {
    pub shapes: Vec<Arc<Hittable>>,
}

impl Hitlist {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
    pub fn add(&mut self, shape: Arc<Hittable>) {
        self.shapes.push(shape);
    }
}

impl Hittable for Hitlist {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let mut rec: Option<Hitrec> = None;
        let mut closest: f64 = t_max;
        for shape in &(self.shapes) {
            match shape.hit(r.clone(), t_min, closest) {
                Some(t_rec) => {
                    closest = t_rec.t;
                    rec = Some(t_rec);
                }
                None => {}
            }
        }
        return rec;
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit<'a>(&'a self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let oc: Vec3 = r.origin() - self.ct();
        let a: f64 = r.diraction().squared_length();
        let h: f64 = (r.diraction() * oc.clone());
        let c: f64 = (oc.squared_length()) - self.rad * self.rad;
        let dis: f64 = h * h - a * c;
        let mut rec: Hitrec = Hitrec::new(&(self.mat));
        if dis <= 0.0 {
            return None;
        } else {
            let root: f64 = dis.sqrt();
            let mut t: f64 = (-h - root) / a;
            if (t > t_min && t < t_max) {
                rec.t = t;
                rec.p = r.at(t);
                let nf: Vec3 = (rec.p() - self.ct()) / self.rad;
                rec.set_face(r.clone(), nf);
                return Some(rec);
            }
            t = (-h + root) / a;
            if (t > t_min && t < t_max) {
                rec.t = t;
                rec.p = r.at(t);
                let nf: Vec3 = (rec.p() - self.ct()) / self.rad;
                rec.set_face(r.clone(), nf);
                return Some(rec);
            }
            return None;
        }
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit<'a>(&'a self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let oc: Vec3 = r.origin() - self.ct(r.time());
        let a: f64 = r.diraction().squared_length();
        let h: f64 = (r.diraction() * oc.clone());
        let c: f64 = (oc.squared_length()) - self.rad * self.rad;
        let dis: f64 = h * h - a * c;
        let mut rec: Hitrec = Hitrec::new(&(self.mat));
        if dis <= 0.0 {
            return None;
        } else {
            let root: f64 = dis.sqrt();
            let mut t: f64 = (-h - root) / a;
            if (t > t_min && t < t_max) {
                rec.t = t;
                rec.p = r.at(t);
                let nf: Vec3 = (rec.p() - self.ct(r.time())) / self.rad;
                rec.set_face(r.clone(), nf);
                return Some(rec);
            }
            t = (-h + root) / a;
            if (t > t_min && t < t_max) {
                rec.t = t;
                rec.p = r.at(t);
                let nf: Vec3 = (rec.p() - self.ct(r.time())) / self.rad;
                rec.set_face(r.clone(), nf);
                return Some(rec);
            }
            return None;
        }
    }
}
