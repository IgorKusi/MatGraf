use std::fmt;
use std::ptr::null;

//TAK JAKBY class Vector2
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2{x,y}
    }


    pub fn length(&self) -> f64{
        f64::sqrt((self.x * self.x) + (self.y * self.y) )
    }
    pub fn add(&self, other: &Vector2) -> Vector2{
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn normalise(&self) -> Vector2{
        if self.length() != 0.0 {
            Vector2{
                x: self.x/ self.length(),
                y: self.y/ self.length(),
            }
        }
        else {
            null()
        }

    }
    pub fn sub(&self, other: &Vector2) -> Vector2{
        Vector2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mag(&self, scalar: &f64 ) -> Vector2{
        Vector2{
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn div(&self, scalar: &f64) -> Vector2{
        self.mag(1.0/&scalar)
    }

    pub fn dot_product(&self, other: Vector2) -> f64{
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn angle(&self, other: Vector2) -> f64{
        let cos = (self.dot_product(other) / (self.length() * other.length()));
        f64::acos(cos)
    }



}
//TAK JAKBY toString
impl fmt::Display for Vector2{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}