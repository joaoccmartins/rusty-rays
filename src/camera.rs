use super::Ray;
use glam::{vec3, Vec3};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub pos: Vec3,
    pub view: Vec3,
    pub sensor_size: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, pos: Vec3, view: Vec3, sensor_size: f32) -> Self {
        Self {
            width,
            height,
            pos,
            view,
            sensor_size,
        }
    }

    // TODO: review mutable requirements of self
    pub fn get_ray(self, x: u32, y: u32, multi_sample_range: f32) -> Ray {
        let half_range = multi_sample_range / 2.0;
        let mut rng = rand::thread_rng();
        let x = x as f32 + rng.gen_range(-half_range..=half_range);
        let y = y as f32 + rng.gen_range(-half_range..=half_range);
        // (0.5,0.5) is top left
        // (-0.5,-0.5) is bottom right
        let normalized = vec3(
            x as f32 / self.width as f32 - 0.5,
            (0.5 - y as f32 / self.height as f32) * self.width as f32 / self.height as f32,
            0.0,
        );
        Ray {
            pos: self.pos + normalized * self.sensor_size,
            dir: self.view,
        }
    }
}
