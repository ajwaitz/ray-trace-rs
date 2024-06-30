// Operator overloading
// https://doc.rust-lang.org/rust-by-example/trait/ops.html
use std::ops::{Add, Sub, Mul, Div, Neg};

use rand;
use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3(x, y, z);
    }

    pub fn x(&self) -> f64 {
        return self.0;
    }

    pub fn y(&self) -> f64 {
        return self.1;
    }

    pub fn z(&self) -> f64 {
        return self.2;
    }

    pub fn sum(&self) -> f64 {
        return self.0 + self.1 + self.2;
    }

    pub fn length_squared(&self) -> f64 {
        return self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn unit(&self) -> Vec3 {
        return (*self).clone() / self.length();
    }

    pub const EMPTY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        return Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        return Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z());
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() - rhs, self.y() - rhs, self.z() - rhs);
    }
}


impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        return Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z());
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs);
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Vec3 {
        return Vec3(self.x() / rhs, self.y() / rhs, self.z() / rhs);
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        return Vec3(-self.x(), -self.y(), -self.z());
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return (a * b).sum();
}

pub fn random_vec3() -> Vec3 {
    return Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
}

pub fn random_range_vec3(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();
    return Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
}

pub fn random_unit_hemisphere_vec3() -> Vec3 {
    while true {
        let p = random_range_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
    return Vec3::new(0.0, 0.0, 0.0);
}

pub fn random_on_hemisphere_vec3(normal: Vec3) -> Vec3 {
    let r = random_unit_hemisphere_vec3();
    return if dot(r, normal) > 0.0 {
        r
    } else {
        -r
    }
}