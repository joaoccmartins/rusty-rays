use dyn_clone::DynClone;
use glam::Vec3;

use crate::ray::{HitResult, Ray};

/// Required Trait for any type that is to be used
/// as a material in the SceneGraph
pub trait Material: DynClone {
    /// How the ray is scattered when a primitive with this material is hit
    fn scatter(&self, hit_result: &HitResult) -> Option<(Ray, Vec3)>;
}

dyn_clone::clone_trait_object!(Material);

/// A material that uses lambertian reflectance for diffusion
#[derive(Copy, Clone, Debug)]
pub struct Diffuse {
    pub albedo: Vec3,
}

impl Material for Diffuse {
    fn scatter(&self, hit_result: &HitResult) -> Option<(Ray, Vec3)> {
        Some((
            Ray::scatter_ray(hit_result.normal, hit_result.pos),
            self.albedo,
        ))
    }
}

/// A material that reflects ligh as a metal
#[derive(Copy, Clone, Debug)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, hit_result: &HitResult) -> Option<(Ray, Vec3)> {
        Some((
            hit_result
                .original_ray
                .reflect_ray(hit_result.normal, hit_result.pos),
            self.albedo,
        ))
    }
}
