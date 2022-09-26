use crate::renderer::geometry::mesh::Mesh;
use crate::renderer::Shader;

pub struct Scene {
    objects: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn iter_meshes(&self) -> core::slice::Iter<Mesh> {
        self.objects.iter()
    }

    pub fn add(&mut self, object: Mesh) {
        self.objects.push(object);
    }

    pub unsafe fn draw(&self) {
        for mesh in &self.objects {
                mesh.draw();
        }
    }
}
