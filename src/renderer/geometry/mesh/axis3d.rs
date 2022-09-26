use super::{Mesh, Vertex};

fn vertices() -> Vec<Vertex> {
    vec![
        [-100., 0.0, 0.0, 1.0, 0.5, 0.5],
        [ 100., 0.0, 0.0, 1.0, 0.5, 0.5],

        [0.0, 100., 0.0, 0.5, 1.0, 0.5],
        [0.0, -100., 0.0, 0.5, 1.0, 0.5],

        [0.0, 0.0,  100., 0.5, 0.5, 1.0],
        [0.0, 0.0, -100., 0.5, 0.5, 1.0],
    ]
}

fn indices() -> Vec<u32> {
    vec![
        0, 1, 1,
        2, 3, 3,
        4, 5, 5
    ]
}

pub fn new() -> Mesh {
    Mesh::new(vertices(), indices())
}