use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
        Vector4 { x, y, z, w }
    }

    pub fn from_pts(p1: (f64, f64, f64, f64), p2: (f64, f64, f64, f64)) -> Vector4 {
        Vector4 {
            x: p2.0 - p1.0,
            y: p2.1 - p1.1,
            z: p2.2 - p1.2,
            w: p2.3 - p1.3,
        }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn add(&self, other: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn normalize(&self) -> Option<Vector4> {
        let len = self.length();
        return if len != 0.0 {
            Some(Vector4 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            })
        } else {
            None
        };
    }

    pub fn sub(&self, other: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector4 {
        Vector4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn divide(&self, scalar: f64) -> Vector4 {
        self.scale(1.0 / scalar)
    }

    pub fn dot(&self, other: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }

    pub fn dot_product(&self, other: &Vector4) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn angle(&self, other: &Vector4) -> f64 {
        let cos = self.dot_product(other) / (self.length() * other.length());
        f64::acos(cos)
    }
}

impl fmt::Display for Vector4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{},{},{}]", self.x, self.y, self.z, self.w)
    }
}
impl Index<usize> for Vector4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Vector4"),
        }
    }
}
impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Vector4"),
        }
    }
}