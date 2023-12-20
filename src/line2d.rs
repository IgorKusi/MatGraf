use crate::vector2::Vector2;

pub struct Line2D {
    pub p1: Vector2,
    pub p2: Vector2,
}

impl Line2D{
    pub fn new_v(p1: Vector2, p2: Vector2) -> Line2D {
        Line2D { p1, p2 }
    }

    pub fn new_p(p1: (f64, f64), p2: (f64, f64)) -> Line2D {
        Line2D { p1: Vector2::new(p1.0, p1.1), p2: Vector2::new(p2.0, p2.1) }
    }

    pub fn do_lines_intersect(&self, other: &Line2D) -> bool {
        let p = &self.p1;
        let q = &other.p1;
        let r = self.p2.sub(&self.p1);
        let s = &other.p2.sub(&other.p1);

        let t = ((&q.sub(&p)).cross(&s)) / (r.cross(&s));
        let u = ((&q.sub(&p)).cross(&r)) / (r.cross(&s));

        return t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0;

    }

    pub fn calculate_angle_between_lines(&self, other: &Line2D) -> f64 {
        let v1 = &self.p2.sub(&self.p1);
        let v2 = &other.p2.sub(&other.p1);
        let dot = v1.dot(&v2);
        let mag1 = v1.length();
        let mag2 = v2.length();
        let angle = f64::acos(dot / (mag1 * mag2));
        return angle.to_degrees();
    }

    pub fn calculate_point_of_intersection(&self, other: &Line2D) -> Option<Vector2> {
        let p = &self.p1;
        let q = &other.p1;
        let r = &self.p2.sub(&self.p1);
        let s = &other.p2.sub(&other.p1);

        let t_numer = (&q.sub(&p)).cross(&s);
        let denom = r.cross(&s);

        // Check if lines are parallel (denom is zero)
        if denom == 0.0 {
            return None;
        }

        let t = t_numer / denom;

        Some(p.add(&r.mag(&t)))
    }


}