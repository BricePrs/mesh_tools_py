use std::mem::size_of;

use super::Vertex;

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            assert_ne!(vao, 0);
            assert_ne!(vbo, 0);
            assert_ne!(ebo, 0);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<Vertex>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);

            gl::BindVertexArray(0);
        }
        Self { vao, vbo, ebo }
    }

    pub unsafe fn draw(&self) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);
    }
}

pub mod cube;
