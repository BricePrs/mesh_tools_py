use crate::renderer::geometry::mesh::{Mesh, Vertex};
use crate::renderer::{scene::object::Object, texture::Texture};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use ultraviolet::{Mat4, Vec3};

#[allow(dead_code)]
pub fn go_to_tag(lines: &mut Lines<BufReader<File>>, tag: &str) -> Vec<String> {
    let mut line;
    loop {
        line = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        if line[0] == tag {
            break;
        }
    }
    line
}

#[allow(dead_code)]
pub fn load_mesh(file_name: &str, scale: f32) -> Object {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let file = File::open(format!("meshes/{}", file_name)).unwrap();
    let mut lines = BufReader::new(file).lines();

    // Go to next element line
    let line = go_to_tag(&mut lines, "element");
    let vertices_count: u32 = line[2].parse().unwrap();

    // Go to next element line
    let line = go_to_tag(&mut lines, "element");
    let indices_count: u32 = line[2].parse().unwrap();

    go_to_tag(&mut lines, "end_header");

    for vertex_index in 0..vertices_count {
        print!("\rLoading file: {}/{vertices_count}", vertex_index + 1);
        let string_values = lines.next().unwrap().unwrap();
        let values = string_values
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let mut vertex_values: Vertex = [1.; 6];
        for i in 0..3 {
            vertex_values[i] = scale * values[i].parse::<f32>().unwrap();
        }
        vertices.push(vertex_values);
    }

    for face_indices in 0..indices_count {
        print!("\rLoading file: {}/{indices_count}", face_indices + 1);
        let string_values = lines.next().unwrap().unwrap();
        let values = string_values
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        for i in 0..3 {
            indices.push(values[i + 1].parse().unwrap());
        }
    }

    let mesh = Mesh::new(vertices, indices);
    let transform = Mat4::from_translation(Vec3::new(0., -30., 20.));
    Object::new(mesh, transform)
}

#[allow(dead_code)]
pub fn load_texture(filename: &str) -> Texture {
    todo!();
}
