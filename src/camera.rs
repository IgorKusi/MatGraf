use crate::cube::Cube;
use crate::vector3::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Camera {
    pub fn cast_ray(&self, x: f64, y: f64) -> Vector3 {
        let ray_direction = self.direction.normalise().unwrap_or(Vector3::new(0.0, 0.0, 1.0));
        let ray_origin = self.origin;
        ray_origin.add(&ray_direction.mag(&x).add(&Vector3::new(0.0, y, 0.0)))
    }
}
pub fn generate_scene(cube: &Cube, camera: &Camera) -> Vec<Vec<char>> {
    let mut scene = vec![vec!['.'; 60]; 60];

    for i in 0..60 {
        for j in 0..60 {
            let ray_origin = camera.origin.clone();
            let ray_direction = if let Some(normalized_direction) = camera.cast_ray(i as f64, j as f64)
                .sub(&ray_origin)
                .normalise() {
                normalized_direction
            } else {
                Vector3::new(0.0, 0.0, 1.0)
            };
            if cube.intersects(&ray_origin, &ray_direction) {
                scene[i][j] = '0';
            }
        }
    }

    scene
}

pub(crate) fn new(p0: Vector3, p1: Vector3) -> Camera {
    Camera { origin: p0, direction: p1 }
}