use glam::Vec3;

#[derive(Clone, Copy)]
pub enum Prim {
    Sphere { pos: Vec3, radius: f32 },
}
