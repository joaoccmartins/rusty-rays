mod material;
mod primitive;

pub(crate) use material::{DiffuseAttributes, Material, MetalAttributes};
pub(crate) use primitive::Prim;

pub type Scene = Vec<(Prim, Material)>;
