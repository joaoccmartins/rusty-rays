use glam::{vec3, Vec3, Vec4};
use image::Rgba;

pub trait Color {
    fn from(self) -> Rgba<u8>;

    fn with_alpha(self, alpha: f32) -> Rgba<u8>;
}

impl Color for Vec4 {
    fn from(self) -> Rgba<u8> {
        Rgba([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (self.w * 255.0) as u8,
        ])
    }

    fn with_alpha(self, alpha: f32) -> Rgba<u8> {
        Rgba([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (alpha * 255.0) as u8,
        ])
    }
}

impl Color for Vec3 {
    fn from(self) -> Rgba<u8> {
        Rgba([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (255.0) as u8,
        ])
    }

    fn with_alpha(self, alpha: f32) -> Rgba<u8> {
        Rgba([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (alpha * 255.0) as u8,
        ])
    }
}

pub fn linear_to_gamma(color: Vec3) -> Vec3 {
    vec3(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
}
