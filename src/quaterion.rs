use std::ops::{Add, Sub, Mul};
use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f64,
    pub v: Vector3,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, v: Vector3::new(x, y, z) }
    }

    pub fn identity() -> Quaternion {
        Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(self.w, -self.v.x, -self.v.y, -self.v.z)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.w * self.w + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z)
    }

    pub fn normalize(&self) -> Quaternion {
        let mag = self.magnitude();
        Quaternion {
            w: self.w / mag,
            v: Vector3 {x: self.v.x / mag, y: self.v.y / mag, z: self.v.z / mag},
        }
    }

    pub fn inverse(self) -> Quaternion {
        if self.magnitude() == 0. {
            panic!();
        }
        return self.conjugate() * (1. / (&self.magnitude() * &self.magnitude()))
    }

    pub fn unit_norm(&self) -> Quaternion {
        let rad = self.w.to_radians();
        let mut v = self.v.normalise().unwrap();
        let w = f64::cos(rad*0.5);
        v = v * f64::sin(rad*0.5);

        return Quaternion{ w, v };
    }

    pub fn rotate(p: Vector3, angle: f64, axis: Vector3) -> Vector3 {
        let pq = Quaternion { w: 0f64, v: p };
        let ax = axis.normalise().unwrap();
        let mut q = Quaternion { w: angle, v: ax };
        q = q.unit_norm();
        let q_inv = q.inverse();
        let rotated = q * pq * q_inv;

        return rotated.v;
    }
}

// Operator overloading for quaternion addition
impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w + other.w,
            v: Vector3 { x: self.v.x + other.v.x, y: self.v.y + other.v.y, z: self.v.z + other.v.z },
        }
    }
}

// Operator overloading for quaternion subtraction
impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w - other.w,
            v: Vector3 { x: self.v.x - other.v.x, y: self.v.y - other.v.y, z: self.v.z - other.v.z },
        }
    }
}

// Operator overloading for quaternion multiplication
impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * other.w - self.v.x * other.v.x - self.v.y * other.v.y - self.v.z * other.v.z,
            v: Vector3 {
                x: self.w * other.v.x + self.v.x * other.w + self.v.y * other.v.z - self.v.z * other.v.y,
                y: self.w * other.v.y - self.v.x * other.v.z + self.v.y * other.w + self.v.z * other.v.x,
                z: self.w * other.v.z + self.v.x * other.v.y - self.v.y * other.v.x + self.v.z * other.w,
            },
        }
    }
}

impl Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion {
            w: self.w * rhs,
            v: Vector3 { x: self.v.x * rhs, y: self.v.y * rhs, z: self.v.z * rhs },
        }
    }
}
