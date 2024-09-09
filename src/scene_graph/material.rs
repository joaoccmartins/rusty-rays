use glam::{vec3, Vec3};
use rand::Rng;

use crate::ray::{HitResult, Ray};

#[derive(Copy, Clone, Debug)]
pub struct DiffuseAttributes {
    pub albedo: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct MetalAttributes {
    pub albedo: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Diffuse(DiffuseAttributes),
    Metal(MetalAttributes),
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

// Generates a random ray in the hemisphere coplanar with the normal
fn get_lambertian_ray(normal: Vec3, pos: Vec3) -> Ray {
    Ray {
        pos,
        dir: pos + normal + random_unit_vector(),
    }
}

// Generates a ray that's a reflection of the preceding ray on the normal
fn reflect_ray(normal: Vec3, pos: Vec3, original: Ray) -> Ray {
    Ray {
        pos,
        dir: original.dir + 2.0 * normal * original.dir.dot(normal),
    }
}

impl Material {
    pub fn scatter(self, hit_result: &HitResult) -> Option<(Ray, Vec3)> {
        match self {
            Material::Diffuse(att) => Some((
                get_lambertian_ray(hit_result.normal, hit_result.pos),
                att.albedo,
            )),
            Material::Metal(att) => Some((
                reflect_ray(hit_result.normal, hit_result.pos, hit_result.original_ray),
                att.albedo,
            )),
            _ => None,
        }
    }
}
