use glam::Vec3;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::ops::Div;

use crate::{
    color::{linear_to_gamma, Color, Framebuffer},
    scene_graph::Scene,
    Camera,
};

use super::core::{hit_scene_with_ray, Renderer};

/// A simple implementation of a Renderer using the core functions
pub struct SimpleRenderer {
    pub camera: Camera,
    pub number_of_samples: u32,
    framebuffer: Framebuffer,
}

impl SimpleRenderer {
    pub fn new(camera: Camera, number_of_samples: u32) -> Self {
        Self {
            camera,
            framebuffer: Framebuffer::new(camera.width as usize, camera.height as usize),
            number_of_samples,
        }
    }
}

impl Renderer for SimpleRenderer {
    /// Renders scene using a single thread.
    fn render(&mut self, scene: &Scene) {
        let number_of_samples = self.number_of_samples;
        let camera = self.camera;
        self.framebuffer
            .par_chunks_exact_mut(self.camera.height as usize)
            .enumerate()
            .for_each(|(y, out)| {
                out.par_iter_mut().enumerate().for_each(|(x, pixel)| {
                    let color: Vec3 = (0..number_of_samples)
                        .map(|_| {
                            hit_scene_with_ray(
                                camera.get_ray(x as u32, y as u32, 1.0),
                                scene,
                                camera.bounce_depth,
                            )
                        })
                        .sum();
                    *pixel = Color::with_alpha(
                        linear_to_gamma(color.div(number_of_samples as f32)),
                        1.0,
                    );
                })
            });
    }

    /// Returns the result framebuffer
    fn framebuffer(&self) -> Framebuffer {
        self.framebuffer.clone()
    }
}
