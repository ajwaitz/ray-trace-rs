use std::fs::File;
use std::io::Write;

// Operator overloading
// https://doc.rust-lang.org/rust-by-example/trait/ops.html
use std::ops::{Add, Mul};

struct Vec3(f64, f64, f64);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3(x, y, z);
    }

    fn x(&self) -> f64 {
        return self.0;
    }

    fn y(&self) -> f64 {
        return self.1;
    }

    fn z(&self) -> f64 {
        return self.2;
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        return Self(self.0, self.1, self.2);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        return Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z());
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

struct Ray {
    origin: Vec3,
    dir: Vec3
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        return self.origin.clone() + self.dir.clone() * t;
    }
}

// Assumes [0,1] input
fn write_color(buf: &mut String, color: Vec3) {
    let r: i64 = (255.0 * color.x()).trunc() as i64;
    let g: i64 = (255.0 * color.y()).trunc() as i64;
    let b: i64 = (255.0 * color.z()).trunc() as i64;
    buf.push_str(format!("{} {} {} ", r, g, b).as_str());
}

fn write_new_line(buf: &mut String) {
    buf.push_str("\n");
}

fn main() {
    let mut file = File::create("test.ppm").unwrap();

    let h = 512;
    let w = 512;

    let mut buf = String::new();

    buf.push_str(format!("P3\n{} {}\n255\n", w, h).as_str());

    for j in 0..h {
        for i in 0..w {
            let c = Vec3(((i as f64) / (w as f64)), ((j as f64) / (h as f64)), 0.0);
            write_color(&mut buf, c);
        }
        write_new_line(&mut buf);
    }

    file.write_all(buf.as_ref()).unwrap();

    println!("Done!");
}
