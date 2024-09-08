use glam::{vec4, Vec3};
use std::ops::Div;

use crate::{
    color::{linear_to_gamma, Framebuffer},
    scene_graph::Scene,
    Camera,
};

use super::core::{hit_scene_with_ray, Renderer};

pub struct SingleThreadedRenderer {
    camera: Camera,
    framebuffer: Framebuffer,
    number_of_samples: u32,
}

impl SingleThreadedRenderer {
    pub fn new(camera: Camera, number_of_samples: u32) -> Self {
        Self {
            camera,
            framebuffer: Framebuffer::new(camera.width as usize, camera.height as usize),
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
                let pixel = linear_to_gamma(color.div(number_of_samples as f32));
                self.framebuffer.put_pixel(
                    x as usize,
                    y as usize,
                    vec4(pixel.x, pixel.y, pixel.z, 1.0),
                );
            })
        });
    }

    fn framebuffer(&self) -> Framebuffer {
        self.framebuffer.clone()
    }
}
