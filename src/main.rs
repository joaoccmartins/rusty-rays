pub(crate) use camera::Camera;
use glam::vec3;
use minifb::{Key, Window, WindowOptions};
use ray::Ray;
use scene_graph::{MetalAttributes, Prim};

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
    // Camera definitions
    let width = 1024;
    let height = 1024;
    let camera = Camera::new(
        width,
        height,
        10,
        20.0,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 1.0),
    );
    let number_of_samples = 10;

    //let mut renderer = SingleThreadedRenderer::new(camera, number_of_samples);
    let mut renderer = MultiThreadedRenderer::new(camera, number_of_samples, 64);

    // Scene definitions
    let vec = vec![
        (
            Prim::Sphere {
                pos: vec3(-1.0, 0.0, -1.2),
                radius: 1.0,
            },
            Material::Diffuse(DiffuseAttributes {
                albedo: vec3(1.0, 0.0, 0.0),
            }),
        ),
        (
            Prim::Sphere {
                pos: vec3(1.0, 0.0, -1.2),
                radius: 1.0,
            },
            Material::Diffuse(DiffuseAttributes {
                albedo: vec3(0.0, 0.0, 1.0),
            }),
        ),
    ];

    let scene: Scene = vec;
    // Minifb Window generation
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

    // The actual render
    renderer.render(&scene);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Push the rendered framebuffer into the window
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
