use crate::vector3::Vector3;
#[derive(Debug, Clone, Copy)]
pub struct Sphere{
    pub(crate) center: Vector3,
    pub(crate) radius: f64,
}

impl Sphere{
    pub fn new(center: (f64, f64, f64), radius: f64) -> Sphere{
        Sphere{
            center: Vector3::new(center.0, center.1, center.2),
            radius,
        }
    }

    pub fn to_string(&self) -> String{
        format!("Sphere: center: {}, radius: {}", self.center.to_string(), self.radius)
    }
}