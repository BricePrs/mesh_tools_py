use gl::types::*;
use std::{ffi, fs::read_to_string};
use ultraviolet::{Mat4, Vec3};

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vertex_source_file: &str, fragment_source_file: &str) -> Self {
        let shader_program;

        unsafe {
            let vertex_shader = Self::create_shader(vertex_source_file, gl::VERTEX_SHADER);
            let fragment_shader = Self::create_shader(fragment_source_file, gl::FRAGMENT_SHADER);

            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut error: Vec<u8> = Vec::with_capacity(1024);
                let mut error_size = 0_i32;
                gl::GetShaderInfoLog(
                    shader_program,
                    1024,
                    &mut error_size,
                    error.as_mut_ptr().cast(),
                );
                error.set_len(error_size.try_into().unwrap());
                panic!(
                    "ERROR::PROGRAM::COULD_NOT_LINK: {}",
                    String::from_utf8_lossy(&error)
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: shader_program }
    }

    fn create_shader(file_path: &str, shader_type: GLenum) -> GLuint {
        let file_path = format!("shaders/{}", file_path);

        let source = read_to_string(&file_path)
            .expect(format!("Could not read file: {}", file_path).as_str());

        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);

            gl::ShaderSource(
                shader,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );

            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut error: Vec<u8> = Vec::with_capacity(1024);
                let mut error_size = 0_i32;
                gl::GetShaderInfoLog(shader, 1024, &mut error_size, error.as_mut_ptr().cast());
                error.set_len(error_size.try_into().unwrap());
                println!(
                    "Error : Could not compile shader : {}",
                    String::from_utf8_lossy(&error)
                );
            }
        }

        shader
    }

    pub fn use_current(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, val: &Mat4) {
        unsafe {
            let c_name = ffi::CString::new(name).unwrap().into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc == -1 {
                println!("Could not find location of uniform : {}", name);
            }
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, val.as_ptr().cast())
        }
    }

    pub fn set_float(&self, name: &str, val: &f32) {
        unsafe {
            let c_name = ffi::CString::new(name).unwrap().into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc == -1 {
                println!("Could not find location of uniform : {}", name);
            }
            gl::Uniform1f(loc, val.clone())
        }
    }

    pub fn set_vec3(&self, name: &str, val: &Vec3) {
        unsafe {
            let c_name = ffi::CString::new(name).unwrap().into_bytes_with_nul();
            let loc = gl::GetUniformLocation(self.id, c_name.as_ptr().cast());
            if loc == -1 {
                //println!("Could not find location of uniform : {}", name);
            }
            gl::Uniform3fv(loc, 1, val.as_ptr().cast())
        }
    }
}
