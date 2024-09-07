use glam::Vec3;
use image::{ImageBuffer, RgbaImage};
use std::ops::Div;

use crate::{
    color::{linear_to_gamma, Color},
    scene_graph::Scene,
    Camera,
};

use super::core::{hit_scene_with_ray, Renderer};

pub struct SingleThreadedRenderer {
    camera: Camera,
    framebuffer: RgbaImage,
    number_of_samples: u32,
}

impl SingleThreadedRenderer {
    pub fn new(camera: Camera, number_of_samples: u32) -> Self {
        Self {
            camera,
            framebuffer: ImageBuffer::new(camera.width, camera.height),
            number_of_samples,
        }
    }
}

impl Renderer for SingleThreadedRenderer {
    fn render(&mut self, scene: &Scene) {
        let number_of_samples = self.number_of_samples;
        let width = self.camera.width;
        let height = self.camera.height;
        let camera = self.camera;
        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                let color: Vec3 = (0..number_of_samples)
                    .map(|_| hit_scene_with_ray(camera.get_ray(x, y, 1.0), scene, 0))
                    .sum();
                self.framebuffer.put_pixel(
                    x,
                    y,
                    Color::with_alpha(linear_to_gamma(color.div(number_of_samples as f32)), 1.0),
                );
            })
        });
    }

    fn framebuffer(&self) -> RgbaImage {
        self.framebuffer.clone()
    }
}
