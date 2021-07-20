use crate::material::Material;
use crate::material::Neg;
use crate::ray::Ray;
use crate::tools;
use crate::vec3::Vec3;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::RemAssign;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Clone, Debug, Copy)]
pub struct Hitrec<'a> {
    pub p: Vec3,
    pub nf: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool, // true: hit outsides
    pub mat: &'a dyn Material,
}

impl<'a> Hitrec<'a> {
    pub fn new(nmat: &'a dyn Material) -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            nf: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
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

pub trait Hittable: Debug {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec>;
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB>;
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
    pub fn get_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + tools::PI;

        *u = phi / (2.0 * tools::PI);
        *v = theta / tools::PI;
    }
    pub fn ct(&self) -> Vec3 {
        self.ct.clone()
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
                Self::get_uv(&rec.nf, &mut rec.u, &mut rec.v);
                return Some(rec);
            }
            t = (-h + root) / a;
            if (t > t_min && t < t_max) {
                rec.t = t;
                rec.p = r.at(t);
                let nf: Vec3 = (rec.p() - self.ct()) / self.rad;
                rec.set_face(r.clone(), nf);
                Self::get_uv(&rec.nf, &mut rec.u, &mut rec.v);
                return Some(rec);
            }
            return None;
        }
    }

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        let tem_v = Vec3::new(self.rad, self.rad, self.rad);
        let out_box = AABB::new(self.ct() - tem_v.clone(), self.ct() + tem_v.clone());
        Some(out_box)
    }
}

#[derive(Debug)]
pub struct MovingSphere<M: Material> {
    pub ct0: Vec3,
    pub ct1: Vec3,
    pub tm0: f64,
    pub tm1: f64,
    pub rad: f64,
    pub mat: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(ct0: Vec3, ct1: Vec3, tm0: f64, tm1: f64, rad: f64, mat: M) -> Self {
        Self {
            ct0,
            ct1,
            tm0,
            tm1,
            rad,
            mat,
        }
    }

    pub fn ct(&self, tm: f64) -> Vec3 {
        self.ct0.clone()
            + (self.ct1.clone() - self.ct0.clone()) * ((tm - self.tm0) / (self.tm1 - self.tm0))
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

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        let tem_v = Vec3::new(self.rad, self.rad, self.rad);
        let box0 = AABB::new(self.ct(t0) - tem_v.clone(), self.ct(t0) + tem_v.clone());
        let box1 = AABB::new(self.ct(t1) - tem_v.clone(), self.ct(t1) + tem_v.clone());
        let out_box = AABB::merge(box0, box1);
        Some(out_box)
    }
}

#[derive(Debug, Clone)]
pub struct AABB {
    mini: Vec3,
    maxi: Vec3,
}

impl AABB {
    pub fn emnew() -> Self {
        let tem_v = Vec3::new(0.0, 0.0, 0.0);
        Self {
            mini: tem_v.clone(),
            maxi: tem_v.clone(),
        }
    }
    pub fn new(mini: Vec3, maxi: Vec3) -> Self {
        Self { mini, maxi }
    }
    pub fn copy(&mut self, other: Self) {
        self.mini.copy(other.min());
        self.maxi.copy(other.max());
    }

    pub fn min(&self) -> Vec3 {
        self.mini.clone()
    }
    pub fn max(&self) -> Vec3 {
        self.maxi.clone()
    }

    fn fmin(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }
    fn fmax(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }

    pub fn merge(box0: Self, box1: Self) -> Self {
        let small = Vec3::new(
            AABB::fmin(box0.min().x(), box1.min().x()),
            AABB::fmin(box0.min().y(), box1.min().y()),
            AABB::fmin(box0.min().z(), box1.min().z()),
        );
        let big = Vec3::new(
            AABB::fmax(box0.max().x(), box1.max().x()),
            AABB::fmax(box0.max().y(), box1.max().y()),
            AABB::fmax(box0.max().z(), box1.max().z()),
        );
        AABB::new(small, big)
    }
    pub fn hit(&self, r: Ray, tmin: f64, tmax: f64) -> bool {
        let mut ti = tmin;
        let mut ta = tmax;
        let mut t0 = 0.0;
        let mut t1 = 0.0;

        let inv_d: f64 = r.diraction().x();
        t0 = (self.min().x() - r.origin().x()) * inv_d;
        t1 = (self.max().x() - r.origin().x()) * inv_d;
        if inv_d < 0.0 {
            let tem = t0;
            t0 = t1;
            t1 = tem;
        }
        ti = if t0 > ti { t0 } else { ti };
        ta = if t1 < ta { t1 } else { ta };
        if ta <= ti {
            return false;
        }

        let inv_d: f64 = r.diraction().y();
        t0 = (self.min().y() - r.origin().y()) * inv_d;
        t1 = (self.max().y() - r.origin().y()) * inv_d;
        if inv_d < 0.0 {
            let tem = t0;
            t0 = t1;
            t1 = tem;
        }
        ti = if t0 > ti { t0 } else { ti };
        ta = if t1 < ta { t1 } else { ta };
        if ta <= ti {
            return false;
        }

        let inv_d: f64 = r.diraction().z();
        t0 = (self.min().z() - r.origin().z()) * inv_d;
        t1 = (self.max().z() - r.origin().z()) * inv_d;
        if inv_d < 0.0 {
            let tem = t0;
            t0 = t1;
            t1 = tem;
        }
        ti = if t0 > ti { t0 } else { ti };
        ta = if t1 < ta { t1 } else { ta };
        if ta <= ti {
            return false;
        }

        return true;
    }
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
            if let Some(t_rec) = shape.hit(r.clone(), t_min, closest) {
                closest = t_rec.t;
                rec = Some(t_rec);
            }
        }
        return rec;
    }

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.shapes.is_empty() {
            return None;
        }

        let neg = Vec3::new(0.0, 0.0, 0.0);
        let mut out_box = AABB::new(neg.clone(), neg.clone());
        let mut flag = true;

        for shape in &(self.shapes) {
            match shape.bebox(t0, t1) {
                Some(t_box) => {
                    out_box.copy(if flag {
                        t_box.clone()
                    } else {
                        AABB::merge(out_box.clone(), t_box.clone())
                    });
                    flag = false;
                }
                None => {
                    return None;
                }
            }
        }

        return Some(out_box);
    }
}

#[derive(Debug, Clone)]
pub struct BvhNode {
    left: Arc<Hittable>,
    right: Arc<Hittable>,
    curbox: AABB,
}

impl BvhNode {
    pub fn new(
        list: &mut Vec<Arc<Hittable>>,
        start: usize,
        end: usize,
        tm0: f64,
        tm1: f64,
    ) -> Self {
        let mut lft = list[start].clone();
        let mut rgt = list[start].clone();
        let axis: i32 = tools::randi(0, 2);

        let span = end - start;

        if span == 1 {
            lft = list[start].clone();
            rgt = list[start].clone();
        } else if span == 2 {
            if BvhNode::box_cmp(&list[start], &list[start + 1], axis) == Ordering::Less {
                lft = list[start].clone();
                rgt = list[start + 1].clone();
            } else {
                lft = list[start + 1].clone();
                rgt = list[start].clone();
            }
        } else {
            list[start..end].sort_by(|a, b| BvhNode::box_cmp(a, b, axis));
            let mid = start + span / 2;
            lft = Arc::new(BvhNode::new(list, start, mid, tm0, tm1));
            rgt = Arc::new(BvhNode::new(list, mid, end, tm0, tm1));
        }

        match lft.bebox(tm0, tm1) {
            Some(box_a) => match rgt.bebox(tm0, tm1) {
                Some(box_b) => Self {
                    left: lft,
                    right: rgt,
                    curbox: AABB::merge(box_a, box_b),
                },
                None => {
                    panic!("No bounding box in bvh_node constructor.");
                }
            },
            None => {
                panic!("No bounding box in bvh_node constructor.");
            }
        }
    }

    pub fn fnew(list: &mut Hitlist, tm0: f64, tm1: f64) -> Self {
        let end: usize = list.shapes.len();
        Self::new(&mut list.shapes, 0, end, tm0, tm1)
    }

    pub fn fcmp(a: f64, b: f64) -> Ordering {
        if a < b {
            return Ordering::Less;
        }
        if a > b {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    pub fn box_cmp(a: &Arc<Hittable>, b: &Arc<Hittable>, axis: i32) -> Ordering {
        match a.bebox(0.0, 0.0) {
            Some(box_a) => match b.bebox(0.0, 0.0) {
                Some(box_b) => match axis {
                    0 => BvhNode::fcmp(box_a.min().x(), box_b.min().x()),
                    1 => BvhNode::fcmp(box_a.min().y(), box_b.min().y()),
                    2 => BvhNode::fcmp(box_a.min().z(), box_b.min().z()),
                    _ => {
                        panic!("Wrong match type.")
                    }
                },
                None => {
                    panic!("No bounding box in bvh_node constructor.");
                }
            },
            None => {
                panic!("No bounding box in bvh_node constructor.");
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        if !self.curbox.hit(r.clone(), t_min, t_max) {
            return None;
        }

        if let Some(rec_l) = self.left.hit(r.clone(), t_min, t_max) {
            match self.right.hit(r.clone(), t_min, rec_l.t) {
                Some(rec_r) => {
                    return Some(rec_r);
                }
                None => {
                    return Some(rec_l);
                }
            }
        }
        match self.right.hit(r.clone(), t_min, t_max) {
            Some(rec_r) => {
                return Some(rec_r);
            }
            None => {
                return None;
            }
        }
    }

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.curbox.clone())
    }
}
