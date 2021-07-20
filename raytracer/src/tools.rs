extern crate rand;

use rand::Rng;

pub const INF: f64 = 9223372036854775807.0;
pub const PI: f64 = 3.1415926535897932385;

pub fn dtr(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn randf(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high)
}

pub fn randi(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low as f64, high as f64 + 1.0) as i32
}

pub fn clamp(x: f64, x_min: f64, x_max: f64) -> f64 {
    if x < x_min {
        return x_min;
    }
    if x > x_max {
        return x_max;
    }
    return x;
}
