use std::time::Instant;

pub(crate) use camera::Camera;
use glam::{vec3, Mat4, Vec3};
use rand::Rng;
use scene_graph::Prim;
pub(crate) use tracing::*;

use renderer::single_threaded::SingleThreadedRenderer;
use renderer::{core::Renderer, multi_threaded::MultiThreadedRenderer};

use crate::scene_graph::{DiffuseAttributes, Material, Scene};

mod camera;
mod color;
mod interval;
mod renderer;
mod scene_graph;
mod tracing;

// Generates a random ray in the hemisphere coplanar with the normal
fn get_ray_in_hemisphere(normal: Vec3, pos: Vec3) -> Ray {
    let mut rng = rand::thread_rng();
    let yaw = rng.gen_range(-90.0_f32.to_radians()..=90.0_f32.to_radians());
    let pitch = rng.gen_range(-90.0_f32.to_radians()..=90.0_f32.to_radians());
    Ray {
        pos,
        dir: Mat4::from_euler(glam::EulerRot::XYZ, yaw, pitch, 0.0).transform_vector3(normal),
    }
}

fn main() {
    let width = 512;
    let height = 512;

    let number_of_samples = 100;

    //let mut renderer = SingleThreadedRenderer::new(
    //    Camera::new(
    //        width,
    //        height,
    //        vec3(0.0, 0.0, -10.0),
    //        vec3(0.0, -0.05, 0.5).normalize(),
    //        1.0,
    //    ),
    //    number_of_samples,
    //);

    let mut renderer = MultiThreadedRenderer::new(
        Camera::new(
            width,
            height,
            vec3(0.0, 0.0, -10.0),
            vec3(0.0, -0.05, 0.5).normalize(),
            1.0,
        ),
        number_of_samples,
        64,
    );

    let vec = vec![
        (
            Prim::Sphere {
                pos: vec3(0.0, 0.1, -10.0),
                radius: 0.1,
            },
            Material::Diffuse(DiffuseAttributes {
                color: vec3(0.5, 0.3, 0.0),
            }),
        ),
        (
            Prim::Sphere {
                pos: vec3(0.0, -10.0, -10.0),
                radius: 10.0,
            },
            Material::Diffuse(DiffuseAttributes {
                color: vec3(0.5, 0.0, 1.8),
            }),
        ),
    ];
    let scene: Scene = vec;

    let start = Instant::now();
    renderer.render(&scene);
    let duration = start.elapsed();
    println!("Frame generated in: {} seconds ", duration.as_secs());
    match renderer.framebuffer().save("new_image.png") {
        Ok(_) => println!("Saved file"),
        Err(err) => println!("Error saving file: {}", err),
    }
}
