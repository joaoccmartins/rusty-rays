use std::time::Instant;

pub(crate) use camera::Camera;
use glam::{vec3, Mat4, Vec3};
use rand::Rng;
use ray::Ray;
use scene_graph::Prim;

use renderer::single_threaded::SingleThreadedRenderer;
use renderer::{core::Renderer, multi_threaded::MultiThreadedRenderer};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::scene_graph::{DiffuseAttributes, Material, Scene};

mod camera;
mod color;
mod interval;
mod ray;
mod renderer;
mod scene_graph;

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

pub fn init_tracing() {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let env = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .with_env_var("RUST_LOG")
        .from_env_lossy();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_level(false)
        .with_thread_ids(true)
        .without_time();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(env)
        .init();
}

fn main() {
    init_tracing();
    let width = 512;
    let height = 512;
    let camera = Camera::new(
        width,
        height,
        vec3(0.0, 0.0, -10.0),
        vec3(0.0, -0.05, 0.5).normalize(),
        1.0,
    );
    let number_of_samples = 100;

    //let mut renderer = SingleThreadedRenderer::new(camera, number_of_samples);

    let mut renderer = MultiThreadedRenderer::new(camera, number_of_samples, 64);

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
    match renderer.framebuffer().save("new_image.png") {
        Ok(_) => tracing::info!("Saved file"),
        Err(err) => tracing::error!("Error saving file: {}", err),
    }
}
