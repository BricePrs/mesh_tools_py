use crate::renderer::geometry::mesh::{Mesh, Vertex};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn load_mesh(file_name: &str) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let file = File::open(format!("meshes/{}", file_name)).unwrap();
    let mut lines = std::io::BufReader::new(file).lines();

    let _ = lines.next();
    let _ = lines.next();
    let _ = lines.next();

    let vertices_count: u32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();

    let _ = lines.next();
    let _ = lines.next();
    let _ = lines.next();

    let indices_count: u32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()[2]
        .parse()
        .unwrap();

    let _ = lines.next();
    let _ = lines.next();

    for _vertex_index in 0..vertices_count {
        let string_values = lines
            .next()
            .unwrap()
            .unwrap();
        let values = string_values
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mut vertex_values: Vertex = [1.; 6];
        for i in 0..3 {
            vertex_values[i] = 0.1_f32*values[i].parse::<f32>().unwrap();
        }
        vertices.push(vertex_values);
    }

    for _face_indices in 0..indices_count {
        let string_values = lines
            .next()
            .unwrap()
            .unwrap();
        let values = string_values
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        for i in 0..3 {
            indices.push(values[i+1].parse().unwrap());
        }
    }

    Mesh::new (vertices, indices)
}

