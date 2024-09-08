use glam::{vec3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct DiffuseAttributes {
    pub color: Vec3,
    pub max_bounce: usize,
}

impl Default for DiffuseAttributes {
    fn default() -> Self {
        Self {
            color: vec3(1.0, 1.0, 1.0),
            max_bounce: 10,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Diffuse(DiffuseAttributes),
}
