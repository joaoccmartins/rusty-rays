use glam::{vec3, Vec3};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    )
    .normalize()
}

impl Ray {
    /// Generates a random ray in the hemisphere coplanar with the normal
    /// using a lambertian scheme
    pub fn scatter_ray(normal: Vec3, pos: Vec3) -> Ray {
        Ray {
            pos,
            dir: pos + normal + random_unit_vector(),
        }
    }

    /// Generates a ray that's a reflection of the preceding ray on the normal
    pub fn reflect_ray(self, normal: Vec3, pos: Vec3) -> Ray {
        Ray {
            pos,
            dir: self.dir + 2.0 * normal * self.dir.dot(normal),
        }
    }
}

pub struct HitResult {
    pub normal: Vec3,
    pub pos: Vec3,
    pub t: f32,
    pub bounce: u32,
    pub original_ray: Ray,
}
