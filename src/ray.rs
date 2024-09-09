use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

pub struct HitResult {
    pub normal: Vec3,
    pub pos: Vec3,
    pub t: f32,
    pub bounce: u32,
    pub original_ray: Ray,
}
