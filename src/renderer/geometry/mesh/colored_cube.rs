use ultraviolet::Mat4;
use crate::renderer::scene::object::Object;
use super::{Mesh, Vertex};

fn vertices() -> Vec<Vertex> {
    vec![
        [-0.5, -0.5, -0.5, 1.0, 0.0, 0.0],
        [0.5, -0.5, -0.5, 0.0, 1.0, 0.0],
        [-0.5, 0.5, -0.5, 0.0, 0.0, 1.0],
        [0.5, 0.5, -0.5, 1.0, 1.0, 0.0],
        [-0.5, -0.5, 0.5, 1.0, 0.0, 1.0],
        [0.5, -0.5, 0.5, 0.0, 1.0, 1.0],
        [-0.5, 0.5, 0.5, 1.0, 1.0, 1.0],
        [0.5, 0.5, 0.5, 0.0, 0.0, 0.0],
    ]
}

fn indices() -> Vec<u32> {
    vec![
        0, 1, 3, 0, 3, 2, 4, 5, 7, 4, 7, 6, 0, 2, 6, 0, 6, 4, 1, 3, 7, 1, 7, 5, 0, 4, 5, 0, 5, 1,
        2, 3, 7, 2, 7, 6,
    ]
}

pub fn new() -> Object {
    let mesh = Mesh::new(vertices(), indices());
    Object::new(mesh, Mat4::identity())
}
