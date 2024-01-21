use crate::cube::Cube;
use crate::vector3::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub direction: Vector3,
    pub yaw: f64,
    pub rotation_speed: f64
}

impl Camera {

    pub fn rotate(&mut self, direction: char) {
        let delta_yaw = self.rotation_speed;

        match direction {
            'A' => self.yaw -= delta_yaw,
            'D' => self.yaw += delta_yaw,
            _ => {}
        }
    }
    pub fn update_direction(&mut self) {
        let direction_x = self.yaw.sin();
        let direction_z = self.yaw.cos();

        self.direction = Vector3::new(direction_x, 0.0, direction_z).normalise().unwrap_or(Vector3::new(0.0, 0.0, 1.0));
    }

    pub fn cast_ray(&mut self, x: f64, y: f64) -> Vector3 {
        self.update_direction();

        let ray_direction = self.direction.normalise().unwrap_or(Vector3::new(0.0, 0.0, 1.0));
        let ray_origin = self.origin;

        let fov = 90.0;
        let aspect_ratio = 1.0;
        let tan_fov = f64::tan((fov / 2.0f64).to_radians());

        let sensor_x = ((x + 0.5) / 60.0) * 2.0 - 1.0;
        let sensor_y = -(((y + 0.5) / 60.0) * 2.0 - 1.0) / aspect_ratio;

        let ray_direction_x = ray_direction.x + sensor_x * tan_fov;
        let ray_direction_y = ray_direction.y + sensor_y * tan_fov;

        let normalized_ray_direction = Vector3::new(ray_direction_x, ray_direction_y, ray_direction.z).normalise().unwrap_or(Vector3::new(0.0, 0.0, 1.0));

        ray_origin.add(&normalized_ray_direction)
    }
}
pub fn generate_scene(cube: &Cube, camera: &mut Camera) -> Vec<Vec<char>> {
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

pub fn new(p0: Vector3, p1: Vector3) -> Camera {
    Camera {
        origin: p0,
        direction: p1,
        rotation_speed: 0.1,
        yaw: 0.0,
    }
}