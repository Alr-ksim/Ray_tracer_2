use crate::color::Color;
use crate::ray::Ray;
use crate::shapes::Hitrec;
use crate::texture;
use crate::texture::Texture;
use crate::tools;
use crate::vec3;
use crate::vec3::Vec3;
use std::cmp::min;
use std::fmt::Debug;
use std::sync::Arc;

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::zero()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Neg {}

impl Neg {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Neg {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Lamber {
    pub lbc: Arc<Texture>,
}

impl Lamber {
    pub fn new(lbc: Arc<Texture>) -> Self {
        Self { lbc }
    }
    pub fn cnew(c: Color) -> Self {
        Self {
            lbc: Arc::new(texture::SolidColor::new(c)),
        }
    }
}

impl Material for Lamber {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        let scat_dir: Vec3 = rec.nf() + vec3::rand_uint_vec();
        scat.copy(Ray::new(rec.p(), scat_dir.clone(), r_in.time()));
        att.copy(self.lbc.value(rec.u, rec.v, &rec.p));
        true
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub lbc: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(lbc: Color, fuzz: f64) -> Self {
        Self { lbc, fuzz }
    }
    pub fn color(&self) -> Color {
        self.lbc.clone()
    }
    pub fn fuz(&self) -> f64 {
        self.fuzz.clone()
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        let rft: Vec3 = Vec3::reflect((r_in.diraction()).unit(), rec.nf());
        scat.copy(Ray::new(
            rec.p(),
            rft.clone() + vec3::rand_in_unit_sphere() * self.fuz(),
            r_in.time(),
        ));
        att.copy(self.color());
        scat.diraction() * rec.nf() > 0.0
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    (r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0))
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
    pub fn rdx(&self) -> f64 {
        self.ref_idx.clone()
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        att.copy(Color::new(1.0, 1.0, 1.0));
        let rate: f64 = if rec.front_face {
            1.0 / self.rdx()
        } else {
            self.rdx()
        };
        let uint_dir: Vec3 = r_in.diraction().unit();
        let tem_cos: f64 = -uint_dir.clone() * rec.nf();
        let cos_theta: f64 = if tem_cos < 1.0 { tem_cos } else { 1.0 };
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if rate * sin_theta > 1.0 {
            let refec: Vec3 = Vec3::reflect(uint_dir.clone(), rec.nf());
            scat.copy(Ray::new(rec.p(), refec.clone(), r_in.time()));
        } else {
            let prob: f64 = schlick(cos_theta, rate);
            if tools::randf(0.0, 1.0) < prob {
                let refec: Vec3 = Vec3::reflect(uint_dir.clone(), rec.nf());
                scat.copy(Ray::new(rec.p(), refec.clone(), r_in.time()));
            } else {
                let refac: Vec3 = Vec3::refract(uint_dir.clone(), rec.nf(), rate);
                scat.copy(Ray::new(rec.p(), refac.clone(), r_in.time()));
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emit: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<Texture>) -> Self {
        Self { emit }
    }
    pub fn cnew(c: Color) -> Self {
        let arc = Arc::new(texture::SolidColor::new(c));
        Self { emit: arc }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Vec3::elemul(self.emit.value(u, v, p), Vec3::ones()) //lighter
    }
}

#[derive(Debug, Clone)]
pub struct Isotropic {
    lbc: Arc<Texture>,
}

impl Isotropic {
    pub fn new(lbc: Arc<Texture>) -> Self {
        Self { lbc }
    }
    pub fn cnew(c: Color) -> Self {
        Self {
            lbc: Arc::new(texture::SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: Ray, rec: Hitrec, att: &mut Color, scat: &mut Ray) -> bool {
        *scat = Ray::new(rec.p(), vec3::rand_in_unit_sphere(), r_in.time());
        *att = self.lbc.value(rec.u, rec.v, &rec.p);
        true
    }
}
