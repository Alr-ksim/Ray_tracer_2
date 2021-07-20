use crate::tools::randf;
use crate::tools::PI;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn copy(&mut self, other: Self) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }
    pub fn x(&self) -> f64 {
        self.x.clone()
    }
    pub fn y(&self) -> f64 {
        self.y.clone()
    }
    pub fn z(&self) -> f64 {
        self.z.clone()
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        (self.squared_length() as f64).sqrt()
    }

    pub fn unit(&self) -> Self {
        let len: f64 = self.length() as f64;
        // if len == 0.0 { panic!("zero vec"); }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn elemul(r1: Self, r2: Self) -> Self {
        Self {
            x: r1.x * r2.x,
            y: r1.y * r2.y,
            z: r1.z * r2.z,
        }
    }

    pub fn cross(r1: Self, r2: Self) -> Self {
        Self {
            x: r1.y * r2.z - r1.z * r2.y,
            y: r1.z * r2.x - r1.x * r2.z,
            z: r1.x * r2.y - r1.y * r2.x,
        }
    }

    pub fn randv() -> Self {
        Self::new(randf(0.0, 1.0), randf(0.0, 1.0), randf(0.0, 1.0))
    }

    pub fn randvr(x_min: f64, x_max: f64) -> Self {
        Self::new(
            randf(x_min, x_max),
            randf(x_min, x_max),
            randf(x_min, x_max),
        )
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v.clone() - n.clone() * ((v.clone() * n.clone()) * 2.0)
    }

    pub fn refract(uv: Self, n: Self, rate: f64) -> Self {
        //rate: etai over etat
        let cos_theta: f64 = (-(uv.clone())) * n.clone();
        let perp: Vec3 = (uv.clone() + n.clone() * cos_theta) * rate;
        let parall: Vec3 = n.clone() * (-((1.0 - perp.squared_length()).abs()).sqrt());
        perp.clone() + parall.clone()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        };
    }
}

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -(self.x),
            y: -(self.y),
            z: -(self.z),
        }
    }
}

pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::randvr(-1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        };
    }
}

pub fn rand_uint_vec() -> Vec3 {
    let a: f64 = randf(0.0, 2.0 * PI);
    let z: f64 = randf(-1.0, 1.0);
    let r: f64 = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

pub fn rand_in_hemisphere(nf: Vec3) -> Vec3 {
    let unit_sphere: Vec3 = rand_in_unit_sphere();
    if (unit_sphere.clone() * nf.clone()) > 0.0 {
        unit_sphere.clone()
    } else {
        -(unit_sphere.clone())
    }
}

pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new(randf(-1.0, 1.0), randf(-1.0, 1.0), 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

// #[cfg(test)]
pub mod tests {
    use super::*;

    //#[test]
    pub fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    //#[test]
    pub fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    //#[test]
    pub fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    //#[test]
    pub fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    // #[test]
    pub fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    //#[test]
    pub fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    //#[test]
    pub fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    //#[test]
    pub fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    //#[test]
    pub fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    // #[test]
    pub fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    // #[test]
    pub fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    // #[test]
    pub fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    // #[test]
    pub fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    // #[test]
    pub fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    // #[test]
    pub fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    // #[test]
    pub fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }

    // #[test]
    pub fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0_f64);
    }

    // #[test]
    pub fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    // #[test]
    pub fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    // #[test]
    // #[should_panic]
    pub fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }
}
