use ultraviolet::{Mat4, Slerp};
use crate::renderer::geometry::mesh::Mesh;

pub struct Object {
    mesh: Mesh,
    transform: Mat4,
}

impl Object {
    pub fn new(mesh: Mesh, transform: Mat4) -> Self {
        Self {
            mesh,
            transform
        }
    }

    pub fn get_transform(&self) -> &Mat4 { &self.transform }
    pub fn get_mesh(&self) -> &Mesh { &self.mesh }
}

