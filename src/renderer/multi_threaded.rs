use std::{
    sync::mpsc::{self, SyncSender},
    thread,
};

use glam::{vec4, Vec3};
use std::ops::Div;

use crate::{
    color::{linear_to_gamma, Color, Framebuffer},
    renderer::core::hit_scene_with_ray,
    scene_graph::Scene,
    Camera,
};

use super::core::Renderer;

// A hyper simple multi threaded version of the renderer, just for expedience sake
// not exactly lightspeed or production quality
pub struct MultiThreadedRenderer {
    pub camera: Camera,
    tiles: Vec<Option<Framebuffer>>,
    number_of_samples: u32,
    tile_size: u32,
}

impl MultiThreadedRenderer {
    pub fn new(camera: Camera, number_of_samples: u32, tile_size: u32) -> Self {
        if camera.width % tile_size != 0 || camera.height % tile_size != 0 {
            todo!("Size needs to be divisible by tile size for now.")
        };
        let columns = camera.width / tile_size;
        let rows = camera.height / tile_size;
        Self {
            camera,
            tiles: vec![None; (columns * rows) as usize],
            number_of_samples,
            tile_size,
        }
    }

    fn render_tile(
        column: u32,
        row: u32,
        scene: Scene,
        camera: &Camera,
        tile_size: u32,
        number_of_samples: u32,
        channel: SyncSender<(u32, u32, Framebuffer)>,
    ) {
        let start_x = column * tile_size;
        let start_y = row * tile_size;
        let mut framebuffer = Framebuffer::new(tile_size as usize, tile_size as usize);
        (start_y..start_y + tile_size).for_each(|y| {
            (start_x..start_x + tile_size).for_each(|x| {
                let color: Vec3 = (0..number_of_samples)
                    .map(|_| hit_scene_with_ray(camera.get_ray(x, y, 1.0), &scene, 0))
                    .sum();
                framebuffer.put_pixel(
                    (x - start_x) as usize,
                    (y - start_y) as usize,
                    Color::with_alpha(linear_to_gamma(color.div(number_of_samples as f32)), 1.0),
                );
            })
        });
        tracing::trace!("Finished Tile ({column}, {row})");
        channel.send((column, row, framebuffer)).unwrap();
    }
}

impl Renderer for MultiThreadedRenderer {
    fn render(&mut self, scene: &Scene) {
        let number_of_samples = self.number_of_samples;
        let camera = self.camera;
        let tile_size = self.tile_size;

        let columns = camera.width / self.tile_size;
        let rows = camera.height / self.tile_size;

        // A channel to receive each of the resulting tile framebuffer
        let (tx, rx) = mpsc::sync_channel((columns * rows) as usize);
        // This is forcefully generating a thread per tile, extra wasteful
        // TODO: substitute for a Threadpool system
        (0..columns).for_each(|column| {
            (0..rows).for_each(|row| {
                let tx_clone = tx.clone();
                let scene_clone = scene.clone();
                thread::spawn(move || {
                    MultiThreadedRenderer::render_tile(
                        column,
                        row,
                        scene_clone,
                        &camera,
                        tile_size,
                        number_of_samples,
                        tx_clone,
                    );
                });
            })
        });
        drop(tx);
        // Collect every tile
        for (column, row, tile) in rx {
            debug_assert!(self.tiles[(column + columns * row) as usize].is_none());
            self.tiles[(column + columns * row) as usize] = Some(tile);
        }
        tracing::trace!("Finished Rendering");
    }

    fn framebuffer(&self) -> Framebuffer {
        let mut framebuffer =
            Framebuffer::new(self.camera.width as usize, self.camera.height as usize);
        let columns = self.camera.width / self.tile_size;
        // Stitch every tile
        for (i, tile) in self.tiles.iter().enumerate() {
            let column = i as u32 % columns;
            let row = i as u32 / columns;
            debug_assert!(tile.is_some());
            if let Some(tile) = tile {
                let start_x = column * self.tile_size;
                let start_y = row * self.tile_size;
                tile.data().iter().enumerate().for_each(|(t, pixel)| {
                    framebuffer.put_pixel(
                        (start_x + t as u32 % self.tile_size) as usize,
                        (start_y + t as u32 / self.tile_size) as usize,
                        *pixel,
                    )
                });
            } else {
                unreachable!()
            }
        }
        tracing::trace!("Finished Stitching");
        framebuffer
    }
}
