use glam::{vec3, Vec3};
use image::RgbaImage;
use rand::Rng;

use crate::{
    interval::Interval,
    ray::{HitResult, Ray},
    scene_graph::{Material, Prim, Scene},
};

pub trait Renderer {
    fn render(&mut self, scene: &Scene);
    fn framebuffer(&self) -> RgbaImage;
}

// Generates a random ray in the hemisphere coplanar with the normal
pub(super) fn get_lambertian_ray(normal: Vec3, pos: Vec3) -> Ray {
    Ray {
        pos,
        dir: pos + normal + random_unit_vector(),
    }
}

pub(super) fn random_unit_vector() -> Vec3 {
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
pub(super) fn get_ray_color(mat: Material, hit: HitResult, scene: &Scene) -> Vec3 {
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

pub(super) fn hit_object_with_ray(
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

pub(super) fn hit_scene_with_ray(ray: Ray, scene: &Scene, bounce_count: usize) -> Vec3 {
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
