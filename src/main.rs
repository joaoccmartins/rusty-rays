pub(crate) use camera::Camera;
use color::{Color, Framebuffer};
use glam::vec3;
use minifb::{Key, Window, WindowOptions};
use ray::Ray;
use scene_graph::Prim;

use renderer::{core::Renderer, multi_threaded::MultiThreadedRenderer};

use crate::scene_graph::{DiffuseAttributes, Material, Scene};

mod camera;
mod color;
mod interval;
mod ray;
mod renderer;
mod scene_graph;

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
    let number_of_samples = 20;

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
                ..Default::default()
            }),
        ),
        (
            Prim::Sphere {
                pos: vec3(0.0, -10.0, -10.0),
                radius: 10.0,
            },
            Material::Diffuse(DiffuseAttributes {
                color: vec3(0.5, 0.0, 1.8),
                ..Default::default()
            }),
        ),
    ];
    let scene: Scene = vec;

    let mut window = Window::new(
        "Rusty Rays Prototype - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    renderer.render(&scene);

    let mut fb = Framebuffer::new(width as usize, height as usize);
    fb.from_fn(|_, _| Color::with_alpha(vec3(0.0, 0.0, 0.0), 0.0));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        window
            .update_with_buffer(
                renderer.framebuffer().data(),
                width as usize,
                height as usize,
            )
            .unwrap();
    }

    match renderer.framebuffer().save("new_image.png") {
        Ok(_) => tracing::info!("Saved file"),
        Err(err) => tracing::error!("Error saving file: {}", err),
    }
}
