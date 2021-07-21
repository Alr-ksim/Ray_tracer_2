#![allow(warnings, unused)]
#![allow(clippy::float_cmp)]
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub mod camera;
pub mod color;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod shapes;
pub mod texture;
pub mod tools;
pub mod vec3;
use camera::Camera;
use color::Color;
use material::Dielectric;
use material::DiffuseLight;
use material::Lamber;
use material::Material;
use material::Metal;
use material::Neg;
use ray::Ray;
use shapes::Hitlist;
use shapes::Hitrec;
use shapes::Hittable;
use shapes::MovingSphere;
use shapes::Sphere;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tools::randf;
use vec3::Vec3;

pub fn ray_color(r: Ray, background: &Color, list: &Hitlist, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match list.hit(r.clone(), 0.001, tools::INF) {
        Some(rec) => {
            let mut scat: Ray = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
            let mut att: Color = Color::zero();
            let emit = rec.mat.emitted(rec.u, rec.v, &rec.p);
            if rec.mat.scatter(r.clone(), rec.clone(), &mut att, &mut scat) {
                return emit.clone()
                    + Color::elemul(
                        att.clone(),
                        ray_color(scat.clone(), background, list, depth - 1),
                    );
            } else {
                emit
            }
        }
        None => *background,
    }
}

pub fn random_scene() -> Hitlist {
    let mut list: Hitlist = Hitlist::new();

    let c1 = Color::new(0.2, 0.3, 0.1);
    let c2 = Color::new(0.9, 0.9, 0.9);
    let mat_g = Arc::new(Lamber::new(Arc::new(texture::CheckerTexture::cnew(c1, c2)))); // 0.5
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
                    let mat = Arc::new(Lamber::cnew(lbc));
                    let ct2: Vec3 = ct.clone() + Vec3::new(0.0, randf(0.0, 0.5), 0.0);
                    let arc_s = Arc::new(MovingSphere::new(
                        ct.clone(),
                        ct2.clone(),
                        0.0,
                        1.0,
                        0.2,
                        mat.clone(),
                    ));
                    list.add(arc_s);
                } else if chmat < 0.95 {
                    let lbc: Color = Color::randvr(0.5, 1.0);
                    let fuzz: f64 = randf(0.0, 0.5); //0.0 -> 0.5
                    let mat = Arc::new(Metal::new(lbc, fuzz));
                    let arc_s = Arc::new(Sphere::new(ct.clone(), 0.2, mat.clone()));
                    list.add(arc_s);
                } else {
                    let mat = Arc::new(Dielectric::new(1.5));
                    let arc_s = Arc::new(Sphere::new(ct.clone(), 0.2, mat.clone()));
                    list.add(arc_s);
                }
            }
            b += 1;
        }
        a += 1;
    }

    let mat_1 = Arc::new(Dielectric::new(1.5));
    let arc_s1 = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat_1.clone()));
    list.add(arc_s1);

    let mat_2 = Arc::new(Lamber::cnew(Color::new(0.4, 0.2, 0.1)));
    let arc_s2 = Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat_2.clone()));
    list.add(arc_s2);

    let mat_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let arc_s3 = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat_3.clone()));
    list.add(arc_s3);

    list
}

pub fn two_sphere() -> Hitlist {
    let mut list = Hitlist::new();

    let c1 = Color::new(0.2, 0.3, 0.1);
    let c2 = Color::new(0.9, 0.9, 0.9);
    let checker = Arc::new(texture::CheckerTexture::cnew(c1, c2));
    let mat = Arc::new(Lamber::new(checker));

    let arc_1 = Arc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, mat.clone()));
    let arc_2 = Arc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, mat.clone()));
    list.add(arc_1);
    list.add(arc_2);

    list
}

pub fn two_perlin() -> Hitlist {
    let mut list = Hitlist::new();

    let pertext = Arc::new(texture::NoiseTexture::new(4.0));
    let mat = Arc::new(Lamber::new(pertext));

    let arc_1 = Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat.clone(),
    ));
    let arc_2 = Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));
    list.add(arc_1);
    list.add(arc_2);

    list
}

pub fn earth() -> Hitlist {
    let mut list = Hitlist::new();
    let path = Path::new("earthmap.jpg");

    let eartext = Arc::new(texture::ImageTexture::new(&path));
    let mat = Arc::new(Lamber::new(eartext));

    let arc_s = Arc::new(Sphere::new(Vec3::zero(), 2.0, mat.clone()));
    list.add(arc_s);

    list
}

pub fn simple_light() -> Hitlist {
    let mut list = Hitlist::new();

    let pertext = Arc::new(texture::NoiseTexture::new(4.0));
    let mat = Arc::new(Lamber::new(pertext));
    let diffmat = Arc::new(DiffuseLight::cnew(Color::new(4.0, 4.0, 4.0)));

    let arc_1 = Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat.clone(),
    ));
    let arc_2 = Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, mat.clone()));
    let arc_3 = Arc::new(shapes::XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffmat.clone(),
    ));
    list.add(arc_1);
    list.add(arc_2);
    list.add(arc_3);

    list
}

pub fn cornell_box() -> Hitlist {
    let mut list = Hitlist::new();

    let red = Arc::new(Lamber::cnew(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lamber::cnew(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lamber::cnew(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::cnew(Color::new(15.0, 15.0, 15.0)));

    let arc_1 = Arc::new(shapes::YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    ));
    let arc_2 = Arc::new(shapes::YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    ));
    let arc_3 = Arc::new(shapes::XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    ));
    let arc_4 = Arc::new(shapes::XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ));
    let arc_5 = Arc::new(shapes::XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    let arc_6 = Arc::new(shapes::XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    let arc_7 = Arc::new(shapes::Boxes::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    let arc_8 = Arc::new(shapes::Boxes::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    ));

    list.add(arc_1);
    list.add(arc_2);
    list.add(arc_3);
    list.add(arc_4);
    list.add(arc_5);
    list.add(arc_6);
    list.add(arc_7);
    list.add(arc_8);

    list
}

fn main() {
    let mut file = File::create("image.ppm").unwrap();

    let mut as_ratio: f64 = 16.0 / 9.0;
    let mut i_wid: i32 = 400;
    let mut i_hit: i32 = (i_wid as f64 / as_ratio) as i32;
    const SAMPLES: i32 = 500; //500
    const MAXDEEP: i32 = 50; //50

    let mut list = Hitlist::new();

    let mut lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let mut lookat = Vec3::new(0.0, 0.0, 0.0);
    let mut vup = Vec3::new(0.0, 1.0, 0.0);
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut dist_to_focus = 10.0;
    let mut backgound = Color::zero();

    const TAC: i32 = 5;
    match TAC {
        0 => {
            list = random_scene();
            backgound = Color::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        1 => {
            list = two_sphere();
            backgound = Color::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        2 => {
            list = two_perlin();
            backgound = Color::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        3 => {
            list = earth();
            backgound = Color::new(0.70, 0.80, 1.00);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        4 => {
            list = simple_light();
            backgound = Color::zero();
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        5 => {
            as_ratio = 1.0;
            i_wid = 600;
            i_hit = 600;

            list = cornell_box();
            backgound = Color::zero();
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
        _ => {}
    }

    let cam: Camera = Camera::new(
        lookfrom.clone(),
        lookat.clone(),
        vup.clone(),
        vfov,
        as_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut img: RgbImage = ImageBuffer::new(i_wid as u32, i_hit as u32);
    let bar = ProgressBar::new(i_hit as u64);

    file.write(format!("P3\n{} {}\n255\n", i_wid, i_hit).as_bytes());
    let mut j: i32 = i_hit - 1;
    while j >= 0 {
        let mut i: i32 = 0;
        while i < i_wid {
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            let mut s: i32 = 0;
            while s < SAMPLES {
                let u: f64 = (i as f64 + randf(0.0, 1.0)) / ((i_wid - 1) as f64);
                let v: f64 = (j as f64 + randf(0.0, 1.0)) / ((i_hit - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                color += ray_color(r, &backgound, &list, MAXDEEP);
                s += 1;
            }
            let pixel = img.get_pixel_mut(i as u32, (i_hit - j - 1) as u32);
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
