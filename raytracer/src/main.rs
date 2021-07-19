#![allow(warnings, unused)]
#![allow(clippy::float_cmp)]
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub mod camera;
pub mod color;
pub mod material;
pub mod ray;
pub mod shapes;
pub mod tools;
pub mod vec3;
use camera::Camera;
use color::Color;
use material::Dielectric;
use material::Lamber;
use material::Material;
use material::Metal;
use material::Neg;
use ray::Ray;
use shapes::Hitlist;
use shapes::Hitrec;
use shapes::Hittable;
use shapes::Sphere;
use shapes::MovingSphere;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use tools::randf;
use vec3::Vec3;

pub fn ray_color(r: Ray, list: &Hitlist, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match list.hit(r.clone(), 0.001, tools::INF) {
        Some(rec) => {
            let tem: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            let mut scat: Ray = Ray::new(tem.clone(), tem.clone(), 0.0);
            let mut att: Color = Color::new(0.0, 0.0, 0.0);
            if rec.mat.scatter(r.clone(), rec.clone(), &mut att, &mut scat) {
                return Color::elemul(att.clone(), ray_color(scat.clone(), list, depth - 1));
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
        None => {
            let unit_dir = r.diraction().unit();
            let t = 0.5 * (unit_dir.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let mut file = File::create("image.ppm").unwrap();

    const AS_RATIO: f64 = 16.0 / 9.0;
    const I_WID: i32 = 400;
    const I_HIT: i32 = (I_WID as f64 / AS_RATIO) as i32;
    const SAMPLES: i32 = 100; //500
    const MAXDEEP: i32 = 50; //50

    let mut img: RgbImage = ImageBuffer::new(I_WID as u32, I_HIT as u32);
    let bar = ProgressBar::new(I_HIT as u64);

    let mut list: Hitlist = Hitlist::new();
    let mat_g: Lamber = Lamber::new(Color::new(0.5, 0.5, 0.5)); // 0.5
    let arc_g = Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_g.clone(),
    ));
    list.add(arc_g);

    let mut a: i32 = -11;
    while a < 11 {
        let mut b: i32 = -11;
        while b < 11 {
            let chmat: f64 = randf(0.0, 1.0);
            let ct: Vec3 = Vec3::new(
                a as f64 + 0.9 * randf(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * randf(0.0, 1.0),
            );

            if (ct.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if chmat < 0.8 {
                    let lbc: Color = Color::elemul(Color::randv(), Color::randv());
                    let mat: Lamber = Lamber::new(lbc);
                    let ct2: Vec3 = ct.clone() + Vec3::new(0.0, randf(0.0, 0.5), 0.0);
                    let arc_s = Arc::new(MovingSphere::new(ct.clone(), ct2.clone(), 0.0, 1.0, 0.2, mat));
                    list.add(arc_s);
                } else if chmat < 0.95 {
                    let lbc: Color = Color::randvr(0.5, 1.0);
                    let fuzz: f64 = randf(0.0, 0.5); //0.0 -> 0.5
                    let mat: Metal = Metal::new(lbc, fuzz);
                    let arc_s = Arc::new(Sphere::new(ct.clone(), 0.2, mat));
                    list.add(arc_s);
                } else {
                    let mat: Dielectric = Dielectric::new(1.5);
                    let arc_s = Arc::new(Sphere::new(ct.clone(), 0.2, mat));
                    list.add(arc_s);
                }
            }
            b += 1;
        }
        a += 1;
    }

    let mat_1: Dielectric = Dielectric::new(1.5);
    let arc_s1 = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_1));
    list.add(arc_s1);

    let mat_2: Lamber = Lamber::new(Color::new(0.4, 0.2, 0.1));
    let arc_s2 = Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_2));
    list.add(arc_s2);

    let mat_3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let arc_s3 = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_3));
    list.add(arc_s3);

    let lookfrom: Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let lookat: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;
    let cam: Camera = Camera::new(
        lookfrom.clone(),
        lookat.clone(),
        vup.clone(),
        20.0,
        AS_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    file.write(format!("P3\n{} {}\n255\n", I_WID, I_HIT).as_bytes());
    let mut j: i32 = I_HIT - 1;
    while j >= 0 {
        let mut i: i32 = 0;
        while i < I_WID {
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            let mut s: i32 = 0;
            while s < SAMPLES {
                let u: f64 = (i as f64 + randf(0.0, 1.0)) / ((I_WID - 1) as f64);
                let v: f64 = (j as f64 + randf(0.0, 1.0)) / ((I_HIT - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                color += ray_color(r, &list, MAXDEEP);
                s += 1;
            }
            let pixel = img.get_pixel_mut(i as u32, (I_HIT - j - 1) as u32);
            let otc: Color = color::out_color(color.clone(), SAMPLES);
            *pixel = image::Rgb([otc.x() as u8, otc.y() as u8, otc.z() as u8]);
            color::write_color(&mut file, color, SAMPLES);
            i += 1;
        }
        bar.inc(1);
        j -= 1;
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
