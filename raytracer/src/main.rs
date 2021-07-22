#![allow(warnings, unused)]
#![allow(clippy::float_cmp)]
use image::GenericImageView;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::clone;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

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

struct World {
    pub height: u32,
}

impl World {
    pub fn new(height: u32) -> Self {
        Self { height }
    }
    pub fn color(&self, _: u32, y: u32) -> u8 {
        (y * 256 / self.height) as u8
    }
}

pub fn ray_color(r: Ray, background: &Color, list: &shapes::BvhNode, depth: i32) -> Color {
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
    let light = Arc::new(DiffuseLight::cnew(Color::new(7.0, 7.0, 7.0)));

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
        113.0,
        443.0,
        127.0,
        432.0,
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
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let arc_7_1 = Arc::new(shapes::RotateY::new(arc_7.clone(), 15.0));
    let arc_7_2 = Arc::new(shapes::Translate::new(
        arc_7_1.clone(),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    let arc_7_3 = Arc::new(shapes::ConstantMedium::cnew(
        arc_7_2.clone(),
        0.01,
        Color::zero(),
    ));

    let arc_8 = Arc::new(shapes::Boxes::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let arc_8_1 = Arc::new(shapes::RotateY::new(arc_8.clone(), -18.0));
    let arc_8_2 = Arc::new(shapes::Translate::new(
        arc_8_1.clone(),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    let arc_8_3 = Arc::new(shapes::ConstantMedium::cnew(
        arc_8_2.clone(),
        0.01,
        Color::ones(),
    ));

    list.add(arc_1);
    list.add(arc_2);
    list.add(arc_3);
    list.add(arc_4);
    list.add(arc_5);
    list.add(arc_6);
    list.add(arc_7_3);
    list.add(arc_8_3);

    list
}

pub fn final_scene() -> Hitlist {
    let mut ground = Hitlist::new();
    let path = Path::new("moonmap.jpg");
    let mat_g = Arc::new(Lamber::cnew(Color::new(0.48, 0.83, 0.53)));
    let mat_g = Arc::new(Lamber::new(Arc::new(texture::ImageTexture::new(&path))));

    let boxes_per_side: usize = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = randf(1.0, 101.0);
            let z1 = z0 + w;

            ground.add(Arc::new(shapes::Boxes::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                mat_g.clone(),
            )));
        }
    }

    let mut list = Hitlist::new();
    list.add(Arc::new(shapes::BvhNode::fnew(&mut ground, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::cnew(Color::new(7.0, 7.0, 7.0)));
    list.add(Arc::new(shapes::XzRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        light.clone(),
    )));

    let ct1 = Vec3::new(400.0, 400.0, 200.0);
    let ct2 = ct1.clone() + Vec3::new(30.0, 0.0, 0.0);
    let moving_mat = Arc::new(Lamber::cnew(Color::new(0.7, 0.3, 0.1)));
    list.add(Arc::new(MovingSphere::new(
        ct1,
        ct2,
        0.0,
        1.0,
        50.0,
        moving_mat.clone(),
    )));

    list.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    list.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    list.add(boundary.clone());
    list.add(Arc::new(shapes::ConstantMedium::cnew(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Vec3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    list.add(Arc::new(shapes::ConstantMedium::cnew(
        boundary.clone(),
        0.0001,
        Color::ones(),
    )));

    let path = Path::new("earthmap.jpg");
    let emat = Arc::new(Lamber::new(Arc::new(texture::ImageTexture::new(&path))));
    list.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));
    let pertext = Arc::new(texture::NoiseTexture::new(0.1));
    list.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lamber::new(pertext.clone())),
    )));

    let mut cube = Hitlist::new();
    let white = Arc::new(Lamber::cnew(Color::new(0.73, 0.73, 0.73)));
    let ns: usize = 1000;
    for i in 0..ns {
        cube.add(Arc::new(Sphere::new(
            Vec3::randvr(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    list.add(Arc::new(shapes::Translate::new(
        Arc::new(shapes::RotateY::new(
            Arc::new(shapes::BvhNode::fnew(&mut cube, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    list
}

pub fn moon() -> Hitlist {
    let mut list = Hitlist::new();
    let path = Path::new("moonmap.jpg");

    let eartext = Arc::new(texture::ImageTexture::new(&path));
    let mat = Arc::new(DiffuseLight::new(eartext));

    let arc_s = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, mat.clone()));
    let arc_s = Arc::new(shapes::RotateY::new(arc_s, 60.0));
    let arc_s = Arc::new(shapes::Translate::new(arc_s, Vec3::new(-2.0, 0.0, -1.0)));
    list.add(arc_s);

    let mat_a = Arc::new(Lamber::new(Arc::new(texture::NoiseTexture::new(0.1))));
    let arc_a = Arc::new(shapes::Boxes::new(Vec3::new(-1.5, -1.5, -1.5), Vec3::new(1.5, 1.5, 1.5), mat_a.clone()));
    let arc_a = Arc::new(shapes::RotateY::new(arc_a, 60.0));
    let arc_a = Arc::new(shapes::RotateZ::new(arc_a, 60.0));
    let arc_a = Arc::new(shapes::Translate::new(arc_a, Vec3::new(0.0, 0.0, 5.0)));
    list.add(arc_a);

    let mat_b = Arc::new(DiffuseLight::cnew(Color::ones()*30.0));
    let arc_b = Arc::new(shapes::XzRect::new(-4.0, 4.0, -4.0, 4.0, 9.0, mat_b.clone()));
    list.add(arc_b);

    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, -1003.0, 0.0),
        1000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    list.add(boundary.clone());
    list.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1003.0, 0.0),
        999.8,
        Arc::new(Lamber::cnew(Color::ones())),
    )));

    list
}
fn main() {
    // let mut file = File::create("image.ppm").unwrap();
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };

    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );

    let mut as_ratio: f64 = 16.0 / 9.0;
    let mut i_wid: i32 = 400;
    let mut i_hit: i32 = (i_wid as f64 / as_ratio) as i32;
    const SAMPLES: i32 = 200; //500
    const MAXDEEP: i32 = 20; //50

    let mut list = Hitlist::new();

    let mut lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let mut lookat = Vec3::new(0.0, 0.0, 0.0);
    let mut vup = Vec3::new(0.0, 1.0, 0.0);
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut dist_to_focus = 10.0;
    let mut backgound = Color::zero();

    const TAC: i32 = 6;
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
        6 => {
            list = final_scene();
            as_ratio = 1.0;
            i_wid = 800;
            i_hit = 800;
            backgound = Color::zero();
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
        }
        7 => {
            i_wid = 800;
            i_hit = 800;
            as_ratio = 1.0;

            list = moon();
            backgound = Color::zero();
            lookfrom = Vec3::new(60.0, 2.0, 0.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
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

    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);

    let bar = ProgressBar::new(n_jobs as u64);

    let world = Arc::new(World::new(i_hit as u32));

    let bvh = shapes::BvhNode::fnew(&mut list, 0.0, 1.0);

    // file.write(format!("P3\n{} {}\n255\n", i_wid, i_hit).as_bytes());
    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let t_list = bvh.clone();
        pool.execute(move || {
            let row_begin = i_hit as usize * i / n_jobs;
            let row_end = i_hit as usize * (i + 1) / n_jobs;
            let rander_height = row_end - row_begin;

            let mut img: RgbImage = ImageBuffer::new(i_wid as u32, rander_height as u32);
            for x in 0..(i_wid as usize) {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = (i_hit as usize - 1 - y) as u32;
                    let mut color: Color = Color::new(0.0, 0.0, 0.0);
                    let mut s: i32 = 0;
                    while s < SAMPLES {
                        let u: f64 = (x as f64 + randf(0.0, 1.0)) / ((i_wid - 1) as f64);
                        let v: f64 = (y as f64 + randf(0.0, 1.0)) / ((i_hit - 1) as f64);
                        let r: Ray = cam.get_ray(u, v);
                        color += ray_color(r, &backgound, &t_list, MAXDEEP);
                        s += 1;
                    }
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);
                    let otc: Color = color::out_color(color.clone(), SAMPLES);
                    *pixel = image::Rgb([otc.x() as u8, otc.y() as u8, otc.z() as u8]);
                    // color::write_color(&mut file, color, SAMPLES);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    let mut img: RgbImage = ImageBuffer::new(i_wid as u32, i_hit as u32);

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..(i_wid as u32) {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
