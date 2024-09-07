use std::{
    ops::Div,
    sync::mpsc::{self, Sender, SyncSender},
    thread,
};

use glam::{vec3, Vec3};
use image::{ImageBuffer, RgbaImage};
use rand::Rng;

use crate::{
    camera::Camera,
    color::{linear_to_gamma, Color},
    interval::Interval,
    scene_graph::{Material, Prim, Scene},
    tracing::{HitResult, Ray},
};

// Generates a random ray in the hemisphere coplanar with the normal
fn get_lambertian_ray(normal: Vec3, pos: Vec3) -> Ray {
    Ray {
        pos,
        dir: pos + normal + random_unit_vector(),
    }
}

fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    )
    .normalize()
}

// Returns the pixel color from material based on the hit
// Might generate more ray hits
fn get_ray_color(mat: Material, hit: HitResult, scene: &Scene) -> Vec3 {
    match mat {
        Material::Diffuse(diffuse_att) => {
            //return (hit.normal + vec3(1.0, 1.0, 1.0)) * 0.5;
            if hit.bounce >= 2 {
                return vec3(0.0, 0.0, 0.0);
            };
            // Generate a random ray on the normal's hemisphere
            0.5 * hit_scene_with_ray(
                get_lambertian_ray(hit.normal, hit.pos),
                scene,
                hit.bounce + 1,
            )
        }
    }
}

fn hit_object_with_ray(
    ray: Ray,
    prim: &Prim,
    interval: Interval,
    bounce_count: usize,
) -> Option<HitResult> {
    match prim {
        Prim::Sphere { pos, radius } => {
            let oc = ray.pos - *pos;
            let a = ray.dir.length_squared();
            let half_b = oc.dot(ray.dir);
            let c = oc.length_squared() - radius * radius;
            let discriminant = half_b * half_b - a * c;

            // sqrt of negative values implies there's no solution
            // to the sphere equation
            if discriminant < 0.0 {
                None
            } else {
                // TODO: Review this branching mess
                let sqrt = f32::sqrt(discriminant);
                let t = if interval.surrounds((-half_b + sqrt) / a) {
                    Some((-half_b + sqrt) / a)
                } else if interval.surrounds((-half_b - sqrt) / a) {
                    Some((-half_b - sqrt) / a)
                } else {
                    None
                };
                t.map(|t| {
                    let hit_pos = ray.pos + ray.dir * t;
                    let normal = (hit_pos - *pos) / *radius;
                    HitResult {
                        normal,
                        pos: hit_pos,
                        t,
                        bounce: bounce_count,
                    }
                })
            }
        }
    }
}

fn hit_scene_with_ray(ray: Ray, scene: &Scene, bounce_count: usize) -> Vec3 {
    if let Some((hit, mat, _prim)) = scene
        .iter()
        .filter_map(|(prim, mat)| {
            hit_object_with_ray(
                ray,
                prim,
                Interval::new(0.0001, f32::INFINITY),
                bounce_count,
            )
            .and_then(|hit| Some((hit, mat, prim)))
        })
        .min_by(|left, right| left.0.t.total_cmp(&right.0.t))
    {
        //hit.normal
        //(hit.normal + vec3(1.0, 1.0, 1.0)) / 0.5
        get_ray_color(*mat, hit, scene)
    } else {
        // Background
        vec3(0.4, 0.6, 0.85)
    }
}

pub struct SingleThreadedRenderer {
    camera: Camera,
    framebuffer: RgbaImage,
    number_of_samples: usize,
}

impl SingleThreadedRenderer {
    pub fn new(camera: Camera, number_of_samples: usize) -> Self {
        Self {
            camera,
            framebuffer: ImageBuffer::new(camera.width, camera.height),
            number_of_samples,
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        let number_of_samples = self.number_of_samples;
        let width = self.camera.width;
        let height = self.camera.height;
        let camera = self.camera;
        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                let color: Vec3 = (0..number_of_samples)
                    .map(|_| {
                        let c = hit_scene_with_ray(camera.get_ray(x, y, 1.0), &scene, 0);
                        c
                    })
                    .sum();
                self.framebuffer.put_pixel(
                    x,
                    y,
                    Color::with_alpha(linear_to_gamma(color.div(number_of_samples as f32)), 1.0),
                );
            })
        });
    }

    pub fn framebuffer(&self) -> RgbaImage {
        self.framebuffer.clone()
    }
}

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
                    .map(|_| {
                        let c = hit_scene_with_ray(camera.get_ray(x, y, 1.0), &scene, 0);
                        c
                    })
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

    pub fn render(&mut self, scene: &Scene) {
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

    pub fn framebuffer(&self) -> RgbaImage {
        let mut framebuffer = ImageBuffer::new(self.camera.width, self.camera.height);
        println!("Started stitching");
        let columns = self.camera.width / self.tile_size;
        let rows = self.camera.height / self.tile_size;
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
