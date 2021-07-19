use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub org: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3) -> Self {
        Ray { org, dir }
    }
    pub fn copy(&mut self, other: Self) {
        self.org.copy(other.origin());
        self.dir.copy(other.diraction());
    }
    pub fn origin(&self) -> Vec3 {
        self.org.clone()
    }
    pub fn diraction(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.org.clone() + self.dir.clone() * t
    }
}
