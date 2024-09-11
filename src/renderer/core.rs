use glam::{vec3, Vec3};

use crate::{
    color::Framebuffer,
    interval::Interval,
    ray::{HitResult, Ray},
    scene_graph::{Material, Prim, Scene},
};

pub trait Renderer {
    fn render(&mut self, scene: &Scene);
    fn framebuffer(&self) -> Framebuffer;
}

/// Returns the pixel color from material based on the hit.
/// Might generate more ray hits
pub(super) fn get_ray_color(mat: &Box<dyn Material>, hit: HitResult, scene: &Scene) -> Vec3 {
    if hit.bounce == 0 {
        return vec3(0.0, 0.0, 0.0);
    };
    // Continue scattering the ray depending on material
    if let Some((ray, albedo)) = mat.scatter(&hit) {
        albedo * hit_scene_with_ray(ray, scene, hit.bounce - 1)
    } else {
        vec3(0.0, 0.0, 0.0)
    }
}

/// Checks the the ray to object intersection
pub(super) fn hit_object_with_ray(
    ray: Ray,
    prim: &Prim,
    interval: Interval,
    bounce_depth: u32,
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
                        bounce: bounce_depth,
                        original_ray: ray,
                    }
                })
            }
        }
    }
}

/// Naively tries to hit every object in the scene to return a color or the background
/// with bounce_depth being the current number of bounces left until we no longer scatter rays
/// TODO: Add background to scene.
pub(super) fn hit_scene_with_ray(ray: Ray, scene: &Scene, bounce_depth: u32) -> Vec3 {
    if let Some((hit, mat)) = find_closest_hit(ray, scene, bounce_depth) {
        get_ray_color(mat, hit, scene)
    } else {
        // Background
        vec3(0.4, 0.6, 0.85)
    }
}

// Find the closest object hit by the ray
fn find_closest_hit(
    ray: Ray,
    scene: &Scene,
    bounce_depth: u32,
) -> Option<(HitResult, &Box<dyn Material>)> {
    scene
        .iter()
        .filter_map(|(prim, mat)| {
            hit_object_with_ray(
                ray,
                prim,
                Interval::new(0.0001, f32::INFINITY),
                bounce_depth,
            )
            .map(|hit| (hit, mat))
        })
        .min_by(|(hit1, _), (hit2, _)| hit1.t.total_cmp(&hit2.t))
}
