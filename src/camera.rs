use std::ops::Div;

use super::Ray;
use glam::{vec3, Vec3};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub bounce_depth: u32,
    pub pos: Vec3,
    // Represents the translation from
    // from the center for a pixel so that
    // f(x,y) = pixel_center + x * pixel_delta_u + y * pixel_delta_v
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_center: Vec3,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        bounce_depth: u32,
        fov: f32,
        look_from: Vec3,
        look_at: Vec3,
    ) -> Self {
        // For now we'll always consider camera up to be world up
        // so no barrel rolls.
        let up = vec3(0.0, 1.0, 0.0);

        let focal_length = (look_at - look_from).length();
        let w = (look_at - look_from) / focal_length;
        let u = up.cross(w).normalize();
        let v = u.cross(w);

        let viewport_height = 2.0 * fov.div(2.0).to_radians().tanh() + focal_length;
        let viewport_width = viewport_height * width as f32 / height as f32;
        println!(
            "({}, {})",
            u * viewport_width / width as f32,
            v * viewport_height / height as f32
        );
        Self {
            width,
            height,
            bounce_depth,
            pos: look_from,
            pixel_delta_u: u * viewport_width / width as f32,
            pixel_delta_v: v * viewport_height / height as f32,
            pixel_center: look_at,
        }
    }

    // TODO: review mutable requirements of self
    pub fn get_ray(self, x: u32, y: u32, multi_sample_range: f32) -> Ray {
        let half_range = multi_sample_range / 2.0;
        let mut rng = rand::thread_rng();
        let x = -(self.width as f32 / 2.0) + x as f32;
        let y = -(self.height as f32 / 2.0) + y as f32;
        let x = x as f32 + rng.gen_range(-half_range..=half_range);
        let y = y as f32 + rng.gen_range(-half_range..=half_range);
        let pixel_position = self.pixel_center + x * self.pixel_delta_u + y * self.pixel_delta_v;
        Ray {
            pos: self.pos,
            dir: (pixel_position - self.pos).normalize(),
        }
    }
}
