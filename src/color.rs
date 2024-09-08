use glam::{vec3, Vec3, Vec4};
use image::{ImageError, Rgba, RgbaImage};

// A very simple framebuffer to be used in conjunction with minifb
#[derive(Clone)]
pub struct Framebuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn data(&self) -> &[u32] {
        &self.data
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: u32) {
        debug_assert!(x < self.width && y < self.height);
        self.data[y * self.height + x] = pixel;
    }

    pub fn save(&self, file_name: &str) -> Result<(), ImageError> {
        RgbaImage::from_fn(self.width as u32, self.height as u32, |x, y| {
            Rgba::from(self.data[(y * self.height as u32 + x) as usize].to_ne_bytes())
        })
        .save(file_name)
    }

    pub fn from_fn<F>(&mut self, mut shader: F)
    where
        F: FnMut(usize, usize) -> u32,
    {
        self.data
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| *p = shader(i % self.width, i / self.height));
    }
}

pub trait Color {
    fn from(self) -> u32;

    fn with_alpha(self, alpha: f32) -> u32;
}

impl Color for Vec4 {
    fn from(self) -> u32 {
        u32::from_ne_bytes([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (self.w * 255.0) as u8,
        ])
    }

    fn with_alpha(self, alpha: f32) -> u32 {
        u32::from_ne_bytes([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (alpha * 255.0) as u8,
        ])
    }
}

impl Color for Vec3 {
    fn from(self) -> u32 {
        u32::from_ne_bytes([
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            (255.0) as u8,
        ])
    }

    fn with_alpha(self, alpha: f32) -> u32 {
        u32::from_ne_bytes([
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
