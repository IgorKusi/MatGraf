use std::fmt;

use vector3::Vector3;
use crate::vector3;

#[derive(Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn from_pts(p1: (f64, f64), p2: (f64, f64)) -> Vector2 {
        Vector2 {
            x: p2.0 - p1.0,
            y: p2.1 - p1.1,
        }
    }


    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn normalise(&self) -> Option<Vector2> {
        let len: f64 = self.length();
        return if len != 0.0 {
            Some(Vector2 {
                x: self.x / len,
                y: self.y / len,
            })
        } else {
            None
        };
    }

    pub fn sub(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mag(&self, scalar: &f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn div(&self, scalar: &f64) -> Vector2 {
        self.mag(&(1.0 / scalar))
    }


    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn dot_product(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle(&self, other: &Vector2) -> f64 {
        let cos = self.dot_product(other) / (self.length() * other.length());
        f64::acos(cos)
    }

    pub fn cross(&self, other: &Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, 0.0)
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}
