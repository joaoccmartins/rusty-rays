use glam::{vec3, Vec3};
use image::{ImageError, Rgba, RgbaImage};
use rayon::slice::{ParallelSlice, ParallelSliceMut};

/// A very simple framebuffer to be used in conjunction with minifb
/// stores data in ARGB format, big endian
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

    /// Returns the data as a slice of u32, each representing a pixel
    pub fn data(&self) -> &[u32] {
        &self.data
    }

    /// Saves the image by blitting it into a RgbaImage from image crate
    /// and using its IO
    pub fn save(&self, file_name: &str) -> Result<(), ImageError> {
        RgbaImage::from_fn(self.width as u32, self.height as u32, |x, y| {
            let pixel_data = self.data[(y * self.height as u32 + x) as usize].to_be_bytes();
            Rgba::from([pixel_data[1], pixel_data[2], pixel_data[3], pixel_data[0]])
        })
        .save(file_name)
    }

    /// Runs a shader function in the framebuffer
    pub fn for_each<F>(&mut self, mut shader: F)
    where
        F: FnMut(usize, usize) -> u32,
    {
        self.data
            .iter_mut()
            .enumerate()
            .for_each(|(i, p)| *p = shader(i % self.width, i / self.height));
    }
}

impl ParallelSlice<u32> for Framebuffer {
    fn as_parallel_slice(&self) -> &[u32] {
        self.data.as_parallel_slice()
    }
}

impl ParallelSliceMut<u32> for Framebuffer {
    fn as_parallel_slice_mut(&mut self) -> &mut [u32] {
        self.data.as_parallel_slice_mut()
    }
}

pub trait Color {
    fn with_alpha(self, alpha: f32) -> u32;
}
impl Color for Vec3 {
    fn with_alpha(self, alpha: f32) -> u32 {
        u32::from_be_bytes([
            (alpha * 255.0) as u8,
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
        ])
    }
}

/// Converts from linear space to gamma corrected space
pub fn linear_to_gamma(color: Vec3) -> Vec3 {
    vec3(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_to_u32_with_alpha() {
        let white = Color::with_alpha(vec3(1.0, 1.0, 1.0), 0.0);
        assert_eq!(white, 0x00FFFFFF);

        let red = Color::with_alpha(vec3(1.0, 0.0, 0.0), 0.0);
        assert_eq!(red, 0x00FF0000);

        let green = Color::with_alpha(vec3(0.0, 1.0, 0.0), 0.0);
        assert_eq!(green, 0x0000FF00);

        let blue = Color::with_alpha(vec3(0.0, 0.0, 1.0), 0.0);
        assert_eq!(blue, 0x000000FF);

        let blue = Color::with_alpha(vec3(0.0, 0.0, 1.0), 1.0);
        assert_eq!(blue, 0xFF0000FF);
    }
}
