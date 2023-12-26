use crate::vector3::Vector3;

pub struct Cube {
    pub center: Vector3,
    pub size: f64,
}

impl Cube {

    pub fn intersects(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> bool {
        let min_bound = self.center.sub(&Vector3::new(self.size / 2.0, self.size / 2.0, self.size / 2.0));
        let max_bound = self.center.add(&Vector3::new(self.size / 2.0, self.size / 2.0, self.size / 2.0));

        let t1 = (min_bound.x - ray_origin.x) / ray_direction.x;
        let t2 = (max_bound.x - ray_origin.x) / ray_direction.x;

        let tmin = f64::min(t1, t2);
        let tmax = f64::max(t1, t2);

        let t3 = (min_bound.y - ray_origin.y) / ray_direction.y;
        let t4 = (max_bound.y - ray_origin.y) / ray_direction.y;

        let tmin = f64::max(tmin, f64::min(t3, t4));
        let tmax = f64::min(tmax, f64::max(t3, t4));

        let t5 = (min_bound.z - ray_origin.z) / ray_direction.z;
        let t6 = (max_bound.z - ray_origin.z) / ray_direction.z;

        let tmin = f64::max(tmin, f64::min(t5, t6));
        let tmax = f64::min(tmax, f64::max(t5, t6));

        tmax > 0.0 && tmin < tmax
    }
}

pub(crate) fn new(p0: Vector3, p1: f64) -> Cube {
    Cube { center: p0, size: p1 }
}