use std::ops::Mul;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::vector3::Vector3;
#[derive(Debug, Clone, Copy)]
pub struct Line3D {
    pub p1: Vector3,
    pub p2: Vector3,
}

impl Line3D{
    pub fn new_v(p1: Vector3, p2: Vector3) -> Line3D {
        Line3D { p1, p2 }
    }

    pub fn new_p(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> Line3D {
        Line3D { p1: Vector3::new(p1.0, p1.1, p1.2), p2: Vector3::new(p2.0, p2.1, p2.2) }
    }

    pub fn do_lines_intersect(&self, other: &Line3D) -> bool {
        let p = &self.p1;
        let q = &other.p1;
        let r = self.p2.sub(&self.p1);
        let s = other.p2.sub(&other.p1);

        let t_numer = (q.sub(&p)).cross(&s);
        let u_numer = (q.sub(&p)).cross(&r);
        let denom = r.cross(&s);

        // Check if lines are parallel (denom is zero)
        if denom.length() == 0.0 {
            return false;
        }

        let t = t_numer.dot(&denom) / (denom.length() * denom.length());
        let u = u_numer.dot(&denom) / (denom.length() * denom.length());

        return t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0;
    }

    pub fn calculate_angle_between_lines(&self, other: &Line3D) -> f64 {
        let v1 = &self.p2.sub(&self.p1);
        let v2 = &other.p2.sub(&other.p1);
        let dot = v1.dot(&v2);
        let mag1 = v1.length();
        let mag2 = v2.length();
        let angle = f64::acos(dot / (mag1 * mag2));
        return angle.to_degrees();
    }

    pub fn calculate_point_of_intersection(&self, other: &Line3D) -> Option<Vector3> {
        let p = &self.p1;
        let q = &other.p1;
        let r = &self.p2.sub(&self.p1);
        let s = &other.p2.sub(&other.p1);

        let t_numer = (&q.sub(&p)).cross(&s);
        let denom = r.cross(&s);

        // Check if lines are parallel (denom is zero)
        if denom.length() == 0.0 {
            // Lines are parallel or coincident, no unique intersection point
            return None;
        }

        let t = t_numer.dot(&denom) / (denom.length() * denom.length());

        Some(p.add(&r.mag(&t)))
    }

    pub fn does_intersect_with_plane(&self, plane: &Plane) -> bool {
        let v1 = &self.p2.sub(&self.p1);
        let v2 = &plane.p2.sub(&plane.p1);
        let normal = plane.calculate_normal(); // Add a method in Plane to calculate the normal vector
        let dot = v1.dot(&normal);
        return dot != 0.0;
    }

    pub fn calculate_point_of_intersection_with_plane(&self, plane: &Plane) -> Option<Vector3> {
        let v1 = &self.p2.sub(&self.p1);
        let normal = plane.calculate_normal(); // Add a method in Plane to calculate the normal vector
        let dot = v1.dot(&normal);

        if dot == 0.0 {
            // The line is parallel to the plane
            return None;
        }

        let t = (&plane.p1.sub(&self.p1)).dot(&normal) / dot;
        Some(self.p1.add(&v1.mul(t)))
    }

    pub fn calculate_angle_between_line_and_plane(&self, plane: &Plane) -> f64 {
        let v1 = &self.p2.sub(&self.p1);
        let v2 = &plane.p2.sub(&plane.p1);
        let dot = v1.dot(&v2);
        let mag1 = v1.length();
        let mag2 = v2.length();
        let angle = f64::acos(dot / (mag1 * mag2));
        return angle.to_degrees();
    }

    pub fn does_intersect_with_sphere(&self, sphere: &Sphere) -> bool {
        let dist = sphere.center.sub(&self.p1).length();
        return dist <= sphere.radius;
    }

    pub fn calculate_point_of_intersection_with_sphere(&self, sphere: &Sphere) -> Option<Vector3> {
        let v1 = &self.p2.sub(&self.p1);
        let dir = v1.normalise()?; // Use normalize to ensure the direction vector is normalized
        let dist = sphere.center.sub(&self.p1).dot(&dir);

        if dist > sphere.radius {
            // The line is outside the sphere
            return None;
        }

        let t = f64::sqrt(sphere.radius * sphere.radius - dist * dist);
        let p = self.p1.add(&dir.mul(t));
        let q = self.p1.add(&dir.mul(-t));

        if p == q {
            // The line is tangent to the sphere
            return Some(p);
        } else {
            // Two intersection points
            return Some(p);
        }
    }

    pub fn display_line(&self) -> String {
        let v1 = &self.p2.sub(&self.p1);
        let a = self.p1.x;
        let b = v1.x;
        let c = self.p1.y;
        let d = v1.y;
        let e = self.p1.z;
        let f = v1.z;
        let x = format!("x = {} + {}t", a, b);
        let y = format!("y = {} + {}t", c, d);
        let z = format!("z = {} + {}t", e, f);
        let line = format!("{}, {}, {}", x, y, z);
        return line;
    }



}