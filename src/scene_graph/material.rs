use glam::Vec3;

use crate::ray::{HitResult, Ray};

/// A material that uses lambertian reflectance for diffusion
#[derive(Copy, Clone, Debug)]
pub struct DiffuseAttributes {
    pub albedo: Vec3,
}

/// A material that reflects ligh as a metal
#[derive(Copy, Clone, Debug)]
pub struct MetalAttributes {
    pub albedo: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Diffuse(DiffuseAttributes),
    Metal(MetalAttributes),
}

impl Material {
    pub fn scatter(self, hit_result: &HitResult) -> Option<(Ray, Vec3)> {
        match self {
            Material::Diffuse(att) => Some((
                Ray::scatter_ray(hit_result.normal, hit_result.pos),
                att.albedo,
            )),
            Material::Metal(att) => Some((
                hit_result
                    .original_ray
                    .reflect_ray(hit_result.normal, hit_result.pos),
                att.albedo,
            )),
            _ => None,
        }
    }
}
