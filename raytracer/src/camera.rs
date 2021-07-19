use crate::ray::Ray;
use crate::tools;
use crate::vec3;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    cw: Vec3,
    lens_radius: f64,
    tm0: f64,
    tm1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _tm0: f64,
        _tm1: f64,
    ) -> Self {
        let theta: f64 = tools::dtr(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = (lookfrom.clone() - lookat.clone()).unit();
        let u: Vec3 = (Vec3::cross(vup.clone(), w.clone())).unit();
        let v: Vec3 = Vec3::cross(w.clone(), u.clone());

        let ori: Vec3 = lookfrom.clone();
        let hor: Vec3 = u.clone() * focus_dist * viewport_width;
        let ver: Vec3 = v.clone() * focus_dist * viewport_height;
        let low: Vec3 =
            ori.clone() - hor.clone() / 2.0 - ver.clone() / 2.0 - w.clone() * focus_dist;
        let len: f64 = aperture / 2.0;

        Self {
            origin: ori.clone(),
            horizontal: hor.clone(),
            vertical: ver.clone(),
            lower_left_corner: low.clone(),
            cu: u.clone(),
            cv: v.clone(),
            cw: w.clone(),
            lens_radius: len.clone(),
            tm0: _tm0,
            tm1: _tm1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = vec3::rand_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.cu.clone() * rd.x() + self.cv.clone() * rd.y();
        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * s
                + self.vertical.clone() * t
                - self.origin.clone()
                - offset.clone(),
                tools::randf(self.tm0, self.tm1),
        )
    }
}
