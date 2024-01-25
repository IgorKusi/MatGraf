use std::fmt;
use std::ops::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_pts(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> Vector3 {
        Vector3 {
            x: p2.0 - p1.0,
            y: p2.1 - p1.1,
            z: p2.2 - p1.2,
        }
    }

    pub fn rotate_around(&self, axis: Vector3, angle: f64) -> Vector3 {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let one_minus_cos_a = 1.0 - cos_a;

        let x = self.x;
        let y = self.y;
        let z = self.z;

        let axis_squared = Vector3::new(axis.x * axis.x, axis.y * axis.y, axis.z * axis.z);

        let mut result = Vector3::new(
            axis_squared.x * (one_minus_cos_a) + cos_a,
            axis.x * axis.y * (one_minus_cos_a) - axis.z * sin_a,
            axis.x * axis.z * (one_minus_cos_a) + axis.y * sin_a,
        );

        result.x *= x;
        result.y *= y;
        result.z *= z;

        result
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn normalise(&self) -> Option<Vector3> {
        let len = self.length();
        if len != 0.0 {
            Some(Vector3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            })
        } else {
            None
        }
    }

    pub fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn mag(&self, scalar: &f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn div(&self, scalar: &f64) -> Vector3 {
        self.mag(&(1.0 / scalar))
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_product(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle(&self, other: &Vector3) -> f64 {
        let cos = self.dot_product(other) / (self.length() * other.length());
        f64::acos(cos)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}
impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}