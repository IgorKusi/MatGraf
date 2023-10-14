use std::fmt;

//TAK JAKBY class Vector2
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2{x,y}
    }

    pub fn add(&self, other: &Vector2) -> Vector2{
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Vector2) -> Vector2{
        Vector2{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scal_mul(&self, scalar: &f64 ) -> Vector2{
        Vector2{
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

}
//TAK JAKBY toString
impl fmt::Display for Vector2{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}
