use crate::color::Color;
use crate::material;
use crate::material::Material;
use crate::material::Neg;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::tools;
use crate::vec3::Vec3;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::RemAssign;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct Hitrec {
    pub p: Vec3,
    pub nf: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool, // true: hit outsides
    pub mat: Arc<Material>,
}

impl Hitrec {
    pub fn new(nmat: Arc<Material>) -> Self {
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
        self.mat = rec.mat.clone();
    }
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec>;
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB>;
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub ct: Vec3,
    pub rad: f64,
    pub mat: Arc<Material>,
}

impl Sphere {
    pub fn new(ct: Vec3, rad: f64, mat: Arc<Material>) -> Self {
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

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let oc: Vec3 = r.origin() - self.ct();
        let a: f64 = r.diraction().squared_length();
        let h: f64 = (r.diraction() * oc.clone());
        let c: f64 = (oc.squared_length()) - self.rad * self.rad;
        let dis: f64 = h * h - a * c;
        let mut rec: Hitrec = Hitrec::new(self.mat.clone());
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

#[derive(Debug, Clone)]
pub struct MovingSphere {
    pub ct0: Vec3,
    pub ct1: Vec3,
    pub tm0: f64,
    pub tm1: f64,
    pub rad: f64,
    pub mat: Arc<Material>,
}

impl MovingSphere {
    pub fn new(ct0: Vec3, ct1: Vec3, tm0: f64, tm1: f64, rad: f64, mat: Arc<Material>) -> Self {
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

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let oc: Vec3 = r.origin() - self.ct(r.time());
        let a: f64 = r.diraction().squared_length();
        let h: f64 = (r.diraction() * oc.clone());
        let c: f64 = (oc.squared_length()) - self.rad * self.rad;
        let dis: f64 = h * h - a * c;
        let mut rec: Hitrec = Hitrec::new(self.mat.clone());
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mat: Arc<Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let t = (self.k - r.origin().z()) / r.diraction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x() + t * r.diraction().x();
        let y = r.origin().y() + t * r.diraction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut rec = Hitrec::new(self.mat.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.set_face(r.clone(), Vec3::new(0.0, 0.0, 1.0));
        rec.p = r.at(t);

        Some(rec)
    }
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        let lit = 0.0001;
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - lit),
            Vec3::new(self.x1, self.y1, self.k + lit),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Arc<Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let t = (self.k - r.origin().y()) / r.diraction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin().x() + t * r.diraction().x();
        let z = r.origin().z() + t * r.diraction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = Hitrec::new(self.mat.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face(r.clone(), Vec3::new(0.0, 1.0, 0.0));
        rec.p = r.at(t);

        Some(rec)
    }
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        let lit = 0.0001;
        Some(AABB::new(
            Vec3::new(self.x0, self.k - lit, self.z0),
            Vec3::new(self.x1, self.k + lit, self.z1),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mat: Arc<Material>,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let t = (self.k - r.origin().x()) / r.diraction().x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin().y() + t * r.diraction().y();
        let z = r.origin().z() + t * r.diraction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = Hitrec::new(self.mat.clone());
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face(r.clone(), Vec3::new(1.0, 0.0, 0.0));
        rec.p = r.at(t);

        Some(rec)
    }
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        let lit = 0.0001;
        Some(AABB::new(
            Vec3::new(self.k - lit, self.y0, self.z0),
            Vec3::new(self.k + lit, self.y1, self.z1),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Boxes {
    box_min: Vec3,
    box_max: Vec3,
    sides: Hitlist,
}

impl Boxes {
    pub fn new(p0: Vec3, p1: Vec3, mat: Arc<Material>) -> Self {
        let mut list = Hitlist::new();

        list.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        list.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )));

        list.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        list.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )));

        list.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        list.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides: list,
        }
    }
}

impl Hittable for Boxes {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        self.sides.hit(r.clone(), t_min, t_max)
    }
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min.clone(), self.box_max.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Translate {
    shape: Arc<Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(shape: Arc<Hittable>, offset: Vec3) -> Self {
        Self { shape, offset }
    }
    pub fn offset(&self) -> Vec3 {
        self.offset.clone()
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let moved_r = Ray::new(r.origin() - self.offset(), r.diraction(), r.time());

        match self.shape.hit(moved_r.clone(), t_min, t_max) {
            Some(mut rec) => {
                rec.p += self.offset();
                rec.set_face(moved_r.clone(), rec.nf());
                Some(rec)
            }
            None => None,
        }
    }
    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self.shape.bebox(t0, t1) {
            Some(out_box) => Some(AABB::new(
                out_box.min() + self.offset(),
                out_box.max() + self.offset(),
            )),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RotateY {
    shape: Arc<Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(arc: Arc<Hittable>, angle: f64) -> Self {
        let radians = tools::dtr(angle);
        let sin_t = radians.sin();
        let cos_t = radians.cos();
        let mut flag = false;
        let mut bx = AABB::emnew();
        if let Some(t_bx) = arc.bebox(0.0, 1.0) {
            bx = t_bx;
            flag = true;
        }

        let mut mn = Vec3::new(tools::INF, tools::INF, tools::INF);
        let mut mx = Vec3::new(-tools::INF, -tools::INF, -tools::INF);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let di = i as f64;
                    let dj = j as f64;
                    let dk = k as f64;
                    let x = di * bx.max().x() + (1.0 - di) * bx.min().x();
                    let y = dj * bx.max().y() + (1.0 - dj) * bx.min().y();
                    let z = dk * bx.max().z() + (1.0 - dk) * bx.min().z();

                    let newx = cos_t * x + sin_t * z;
                    let newz = -sin_t * x + cos_t * z;

                    let tes = Vec3::new(newx, y, newz);
                    mn.x = if mn.x < tes.x { mn.x } else { tes.x };
                    mx.x = if mx.x > tes.x { mx.x } else { tes.x };
                    mn.y = if mn.y < tes.y { mn.y } else { tes.y };
                    mx.y = if mx.y > tes.y { mx.y } else { tes.y };
                    mn.z = if mn.z < tes.z { mn.z } else { tes.z };
                    mx.z = if mx.z > tes.z { mx.z } else { tes.z };
                }
            }
        }

        Self {
            shape: arc,
            sin_theta: sin_t,
            cos_theta: cos_t,
            hasbox: flag,
            bbox: AABB::new(mn, mx),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        let mut org = r.origin();
        let mut dir = r.diraction();
        let sin = self.sin_theta;
        let cos = self.cos_theta;

        org.x = cos * r.origin().x() - sin * r.origin().z();
        org.z = sin * r.origin().x() + cos * r.origin().z();

        dir.x = cos * r.diraction().x() - sin * r.diraction().z();
        dir.z = sin * r.diraction().x() + cos * r.diraction().z();

        let ror = Ray::new(org, dir, r.time());

        match self.shape.hit(ror.clone(), t_min, t_max) {
            Some(mut rec) => {
                let mut p = rec.p();
                let mut nf = rec.nf();

                p.x = cos * rec.p().x() + sin * rec.p().z();
                p.z = -sin * rec.p().x() + cos * rec.p().z();
                nf.x = cos * rec.nf().x() + sin * rec.nf().z();
                nf.z = -sin * rec.nf().x() + cos * rec.nf().z();

                rec.p = p;
                rec.set_face(ror.clone(), nf);

                Some(rec)
            }
            None => None,
        }
    }

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.hasbox {
            Some(self.bbox.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Arc<Hittable>,
    phase_function: Arc<Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Arc<Hittable>, d: f64, a: Arc<Texture>) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(material::Isotropic::new(a)),
            neg_inv_density: (-1.0 / d),
        }
    }
    pub fn cnew(b: Arc<Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(material::Isotropic::cnew(c)),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrec> {
        const DEBUGABLE: bool = false;
        let debuging: bool = DEBUGABLE && (tools::randf(0.0, 1.0) < 0.00001);

        match self.boundary.hit(r.clone(), -tools::INF, tools::INF) {
            Some(mut rec_1) => match self.boundary.hit(r.clone(), rec_1.t + 0.0001, tools::INF) {
                Some(mut rec_2) => {
                    if debuging {
                        print!("\nt_min = {} , t_max = {}\n", rec_1.t, rec_2.t);
                    }

                    if rec_1.t < t_min {
                        rec_1.t = t_min;
                    }
                    if rec_2.t > t_max {
                        rec_2.t = t_max;
                    }

                    if rec_1.t >= rec_2.t {
                        return None;
                    }

                    if rec_1.t < 0.0 {
                        rec_1.t = 0.0;
                    }

                    let ray_len = r.diraction().length();
                    let dis_in_boundary = (rec_2.t - rec_1.t) * ray_len;
                    let hit_dis = self.neg_inv_density * tools::randf(0.0, 1.0).ln();

                    if hit_dis > dis_in_boundary {
                        return None;
                    }

                    let mut rec = Hitrec::new(self.phase_function.clone());
                    rec.t = rec_1.t + hit_dis / ray_len;
                    rec.p = r.at(rec.t);

                    if debuging {
                        print!(
                            "hit_dis = {}\nrec.t = {}\nrec.p = {:?}\n",
                            hit_dis,
                            rec.t,
                            rec.p()
                        );
                    }

                    rec.nf = Vec3::new(1.0, 0.0, 0.0);
                    rec.front_face = true;

                    Some(rec)
                }
                None => None,
            },
            None => None,
        }
    }

    fn bebox(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bebox(t0, t1)
    }
}
