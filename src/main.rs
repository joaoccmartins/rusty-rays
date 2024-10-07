pub(crate) use camera::Camera;
use glam::vec3;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use ray::Ray;

use renderer::{core::Renderer, simple_renderer::SimpleRenderer};
use scene_graph::{Diffuse, Metal};

use crate::scene_graph::{Prim, Scene};

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
        .with_default_directive(LevelFilter::INFO.into())
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
    let width = 512;
    let height = 512;
    let mut look_from = vec3(0.0, 0.0, 2.0);
    let mut look_at = vec3(0.0, 0.0, 1.0);
    let up = vec3(0.0, 1.0, 0.0);
    let bounce_depth = 50;
    let fov = 20.0;
    let camera = Camera::new(width, height, bounce_depth, fov, look_from, look_at, up);
    let number_of_samples = 100;

    let mut renderer = SimpleRenderer::new(camera, number_of_samples);

    // Scene definitions
    let mut scene = Scene::new();
    scene.add(
        Prim::Sphere {
            pos: vec3(-1.0, 0.0, -1.2),
            radius: 1.0,
        },
        Diffuse {
            albedo: vec3(1.0, 0.0, 0.0),
        },
    );
    scene.add(
        Prim::Sphere {
            pos: vec3(1.0, 0.0, -1.2),
            radius: 1.0,
        },
        Metal {
            albedo: vec3(0.0, 0.0, 1.0),
        },
    );

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

    // The window loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Push the rendered framebuffer into the window
        window
            .update_with_buffer(
                renderer.framebuffer().data(),
                width as usize,
                height as usize,
            )
            .unwrap();

        // Check for keyboard inputs and change camera
        let step_size = 0.1;
        let view = (look_at - look_from).normalize();
        let mut camera_translation = vec3(0.0, 0.0, 0.0);
        window
            .get_keys_pressed(KeyRepeat::Yes)
            .iter()
            .for_each(|key| match key {
                Key::W => camera_translation += view * step_size,
                Key::A => camera_translation += view.cross(up) * step_size,
                Key::S => camera_translation -= view * step_size,
                Key::D => camera_translation -= view.cross(up) * step_size,
                _ => {}
            });

        // If we've just moved...
        if camera_translation.length() >= f32::EPSILON {
            look_from += camera_translation;
            look_at += camera_translation;
            // ...let's limit the bounce_depth  and number of samples to 1
            renderer.camera = Camera::new(width, height, 1, fov, look_from, look_at, up);
            renderer.number_of_samples = 1;
            renderer.render(&scene);
        }
        // If the number of samples is 1, then we moved last frame...
        else if renderer.number_of_samples != number_of_samples {
            // ...so we want to render a new version with higher samples count and bounce_depth;
            renderer.camera = Camera::new(width, height, bounce_depth, fov, look_from, look_at, up);
            renderer.number_of_samples = number_of_samples;
            renderer.render(&scene);
        }
    }

    // Let's save the last rendered frame to disk
    match renderer.framebuffer().save("new_image.png") {
        Ok(_) => tracing::info!("Saved file"),
        Err(err) => tracing::error!("Error saving file: {}", err),
    }
}
