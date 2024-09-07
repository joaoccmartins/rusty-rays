use std::{
    sync::mpsc::{self, SyncSender},
    thread,
};

use glam::Vec3;
use image::{ImageBuffer, RgbaImage};
use std::ops::Div;

use crate::{
    color::{linear_to_gamma, Color},
    renderer::core::hit_scene_with_ray,
    scene_graph::Scene,
    Camera,
};

use super::core::Renderer;

pub struct MultiThreadedRenderer {
    camera: Camera,
    tiles: Vec<Option<RgbaImage>>,
    number_of_samples: u32,
    tile_size: u32,
}

impl MultiThreadedRenderer {
    pub fn new(camera: Camera, number_of_samples: u32, tile_size: u32) -> Self {
        if camera.width % tile_size != 0 || camera.height % tile_size != 0 {
            todo!()
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
        channel: SyncSender<(u32, u32, RgbaImage)>,
    ) {
        let start_x = column * tile_size;
        let start_y = row * tile_size;
        let mut framebuffer = ImageBuffer::new(tile_size, tile_size);
        (start_y..start_y + tile_size).for_each(|y| {
            (start_x..start_x + tile_size).for_each(|x| {
                let color: Vec3 = (0..number_of_samples)
                    .map(|_| hit_scene_with_ray(camera.get_ray(x, y, 1.0), &scene, 0))
                    .sum();
                framebuffer.put_pixel(
                    x - start_x,
                    y - start_y,
                    Color::with_alpha(linear_to_gamma(color.div(number_of_samples as f32)), 1.0),
                );
            })
        });

        println!("\t\tFinished rendering tile ({column}, {row})");
        channel.send((column, row, framebuffer)).unwrap()
    }
}

impl Renderer for MultiThreadedRenderer {
    fn render(&mut self, scene: &Scene) {
        let number_of_samples = self.number_of_samples;
        let camera = self.camera;
        let tile_size = self.tile_size;

        let columns = camera.width / self.tile_size;
        let rows = camera.height / self.tile_size;

        let (tx, rx) = mpsc::sync_channel((columns * rows) as usize);

        (0..columns).for_each(|column| {
            (0..rows).for_each(|row| {
                println!("Spawning thread for tile ({column}, {row})");
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
        for (column, row, tile) in rx {
            println!("\tReceiving tile ({column}, {row})");
            self.tiles[(column + columns * row) as usize] = Some(tile);
        }
        println!("Finished render");
    }

    fn framebuffer(&self) -> RgbaImage {
        let mut framebuffer = ImageBuffer::new(self.camera.width, self.camera.height);
        println!("Started stitching");
        let columns = self.camera.width / self.tile_size;
        for (i, tile) in self.tiles.iter().enumerate() {
            let column = i as u32 / columns;
            let row = i as u32 % columns;
            if let Some(tile) = tile {
                let start_x = column * self.tile_size;
                let start_y = row * self.tile_size;
                tile.pixels().enumerate().for_each(|(t, pixel)| {
                    framebuffer.put_pixel(
                        start_x + t as u32 / self.tile_size,
                        start_y + t as u32 % self.tile_size,
                        *pixel,
                    )
                });
            } else {
                println!("Tile ({}, {}) was not rendered", column, row);
            }
        }
        println!("Ended stitching");
        framebuffer
    }
}
