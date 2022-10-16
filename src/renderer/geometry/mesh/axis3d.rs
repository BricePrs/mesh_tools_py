use super::{Mesh, Vertex};
use crate::renderer::scene::object::Object;
use ultraviolet::Mat4;

fn vertices() -> Vec<Vertex> {
    vec![
        [-100., 0.0, 0.0, 1.0, 0.5, 0.5],
        [100., 0.0, 0.0, 1.0, 0.5, 0.5],
        [0.0, 100., 0.0, 0.5, 1.0, 0.5],
        [0.0, -100., 0.0, 0.5, 1.0, 0.5],
        [0.0, 0.0, 100., 0.5, 0.5, 1.0],
        [0.0, 0.0, -100., 0.5, 0.5, 1.0],
    ]
}

fn indices() -> Vec<u32> {
    vec![0, 1, 1, 2, 3, 3, 4, 5, 5]
}

pub fn new() -> Object {
    let mesh = Mesh::new(vertices(), indices());
    Object::new(mesh, Mat4::identity())
}
