use std::ffi::c_void;

use gl::types::GLenum;

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new(file_name: &str) -> Self {
        let buffer = image::open(format!("textures/{}", file_name))
            .unwrap()
            .flipv()
            .to_rgba8();

        let size = buffer.dimensions();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set the texture wrapping/filtering options (on the currently bound texture object)
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
            // load and generate the texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA.try_into().unwrap(),
                size.0.try_into().unwrap(),
                size.1.try_into().unwrap(),
                0,
                gl::RGBA.try_into().unwrap(),
                gl::UNSIGNED_BYTE,
                buffer.as_raw().as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);

        }
        Self { id }
    }
    pub unsafe fn bind(&self, texture_index: GLenum) {
        gl::ActiveTexture(texture_index);
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
}
