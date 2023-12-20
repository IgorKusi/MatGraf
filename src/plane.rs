use crate::line3D::Line3D;
use crate::vector3::Vector3;
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub p1: Vector3,
    pub p2: Vector3,
    pub p3: Vector3,
}

impl Plane{
    pub fn new_v(p1: Vector3, p2: Vector3, p3: Vector3) -> Plane {
        Plane { p1, p2, p3 }
    }

    pub fn new_p(p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> Plane {
        Plane { p1: Vector3::new(p1.0, p1.1, p1.2), p2: Vector3::new(p2.0, p2.1, p2.2), p3: Vector3::new(p3.0, p3.1, p3.2) }
    }

    pub fn calculate_normal(&self) -> Vector3 {
        let v1 = &self.p2.sub(&self.p1);
        let v2 = &self.p3.sub(&self.p1);
        let cross = v1.cross(&v2);
        return cross;
    }

    pub fn does_intersect_with_plane(&self, other: &Plane) -> bool {
        let normal1 = self.calculate_normal();
        let normal2 = other.calculate_normal();
        let dot = normal1.dot(&normal2);
        return dot != 1.0;
    }

    pub fn calculate_line_of_intersection_with_plane(&self, other: &Plane) -> Option<Line3D> {
        let normal1 = self.calculate_normal();
        let normal2 = other.calculate_normal();
        let cross = normal1.cross(&normal2);
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;
        let p4 = other.p1;
        let p5 = other.p2;
        let p6 = other.p3;
        let denom = cross.dot(&cross);
        let t = ((p1.sub(&p4)).dot(&cross)) / denom;
        let u = ((p4.sub(&p1)).dot(&cross)) / denom;
        let p = p1.add(&p2.sub(&p1).mag(&t));
        let q = p4.add(&p5.sub(&p4).mag(&u));
        let r = p2.add(&p3.sub(&p2).mag(&t));
        let s = p5.add(&p6.sub(&p5).mag(&u));
        let line = Line3D::new_v(p, q);
        if line.do_lines_intersect(&Line3D::new_v(r, s)) {
            return Some(line);
        } else {
            return None;
        }

    }

    pub fn calculate_angle_between_planes(&self, other: &Plane) -> f64 {
        let normal1 = self.calculate_normal();
        let normal2 = other.calculate_normal();
        let dot = normal1.dot(&normal2);
        let mag1 = normal1.length();
        let mag2 = normal2.length();
        let angle = f64::acos(dot / (mag1 * mag2));
        return angle.to_degrees();
    }



}