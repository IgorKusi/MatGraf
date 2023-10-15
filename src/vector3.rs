use std::fmt;

#[allow(dead_code)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3{
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{x,y,z}
    }

    pub fn length(&self) -> f64{
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z )
    }

    pub fn add(&self, other: &Vector3) -> Vector3{
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn normalise(&self) -> Vector3{
        if self.length() != 0.0 {
            Vector3{
                x: self.x/ self.length(),
                y: self.y/ self.length(),
                z: self.z/ self.length(),
            }
        }
        else {
            //TODO zmienic to zeby zwracalo nulla
            Vector3{
                x: 0.0,
                y: 0.0,
                z: 0.0,

            }
        }

    }

    pub fn sub(&self, other: &Vector3) -> Vector3{
        Vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn mag(&self, scalar: &f64 ) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn div(&self, scalar: &f64) -> Vector3{
        self.mag(&(1.0/scalar))
    }

    pub fn dot_product(&self, other: &Vector3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle(&self, other: &Vector3) -> f64{
        let cos = self.dot_product(other) / (self.length() * other.length());
        f64::acos(cos)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3{
        Vector3{
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

}

impl fmt::Display for Vector3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}