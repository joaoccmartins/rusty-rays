use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct DiffuseAttributes {
    pub color: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Diffuse(DiffuseAttributes),
}
