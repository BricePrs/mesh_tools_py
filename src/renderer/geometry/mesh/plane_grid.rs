use super::{Mesh, Vertex};
use crate::renderer::scene::object::Object;
use ultraviolet::Mat4;

static LINE_NBR: i32 = 500;
static COLOR: f32 = 0.5;
static SCALE: f32 = 0.2;

fn vertices() -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for i in -LINE_NBR..=LINE_NBR {
        if i == 0 {
            continue;
        }

        let x = 2. * i as f32 * SCALE;
        let y = 2. * LINE_NBR as f32 * SCALE;

        vertices.push([x, 0., y, COLOR, COLOR, COLOR]);
        vertices.push([x, 0., -y, COLOR, COLOR, COLOR]);

        vertices.push([y, 0., x, COLOR, COLOR, COLOR]);
        vertices.push([-y, 0., x, COLOR, COLOR, COLOR]);
    }
    vertices
}

fn indices() -> Vec<u32> {
    let mut indices = Vec::new();
    for i in 0..(2 * LINE_NBR) {
        indices.push((4 * i) as u32);
        indices.push((4 * i + 1) as u32);
        indices.push((4 * i + 1) as u32);

        indices.push((4 * i + 2) as u32);
        indices.push((4 * i + 3) as u32);
        indices.push((4 * i + 3) as u32);
    }
    indices
}

pub fn new() -> Object {
    let mesh = Mesh::new(vertices(), indices());
    Object::new(mesh, Mat4::identity())
}
