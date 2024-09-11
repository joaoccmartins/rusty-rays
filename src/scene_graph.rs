mod material;
mod primitive;

pub(crate) use material::{Diffuse, Material, Metal};
pub(crate) use primitive::Prim;

type SceneNode = (Prim, Box<dyn Material>);
type SceneStorage = Vec<SceneNode>;

#[derive(Clone)]
pub struct Scene {
    storage: SceneStorage,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn add<T>(&mut self, primitive: Prim, material: T)
    where
        T: Material + 'static,
    {
        self.storage.push((primitive, Box::new(material)));
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (Prim, Box<dyn Material>)> {
        self.storage.iter()
    }
}
