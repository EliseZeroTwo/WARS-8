use crate::RAND_SRC;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

pub fn abs(x: f32) -> i32 {
    x.trunc() as i32
}

pub fn atan2(dx: f32, dy: f32) -> f32 {
    dx.atan2(dy)
}

pub fn band(x: i32, y: i32) -> i32 {
    x & y
}

pub fn bnot(x: i32) -> i32 {
    !x
}

pub fn bor(x: i32, y: i32) -> i32 {
    x | y
}

pub fn bxor(x: i32, y: i32) -> i32 {
    x ^ y
}

pub fn cos(x: f32) -> f32 {
    ((std::f32::consts::PI * 2.0) / x).cos()
}

pub fn flr(x: f32) -> f32 {
    x.floor()
}

pub fn min(x: i32, y: i32) -> i32 {
    std::cmp::min(x, y)
}

pub fn minf(x: f32, y: f32) -> f32 {
    if x > y {
        y
    } else {
        x
    }
}

pub fn max(x: i32, y: i32) -> i32 {
    std::cmp::max(x, y)
}

pub fn maxf(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn mid(x: i32, y: i32, z: i32) -> i32 {
    (x + y + z) - min(min(x, y), z) - max(max(x, y), z)
}

pub fn midf(x: f32, y: f32, z: f32) -> f32 {
    (x + y + z) - minf(minf(x, y), z) - maxf(maxf(x, y), z)
}

pub fn rnd(x: i32) -> i32 {
    RAND_SRC.lock().unwrap().gen::<i32>() * x
}

pub fn rndf(x: f32) -> f32 {
    RAND_SRC.lock().unwrap().gen::<f32>() * x
}

pub fn srand(x: i32) {
    let u_x = x as u128;
    let seed: u128 = (u_x << 12) | (u_x << 8) | (u_x << 4) | (u_x);
    *RAND_SRC.lock().unwrap() = Pcg64Mcg::new(seed);
}

pub fn sgn(x: i32) -> i32 {
    if x < 0 {
        -1
    } else {
        1
    }
}

pub fn sgnf(x: f32) -> i32 {
    if x < 0.0 {
        -1
    } else {
        1
    }
}

pub fn shl(x: i32, y: i32) -> i32 {
    x << y
}

pub fn shr(x: i32, y: i32) -> i32 {
    y >> x
}

pub fn sin(x: f32) -> f32 {
    (((std::f32::consts::PI * 2.0) * -x).sin()).max(-1.0).min(1.0)
}

pub fn sqrt(x: i32) -> f32 {
    (x as f32).sqrt()
}

pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
