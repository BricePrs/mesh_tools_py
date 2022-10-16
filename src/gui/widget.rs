use crate::renderer::texture::Texture;
use gl::types::GLenum;
use ultraviolet::IVec2;
use crate::renderer::Shader;

pub struct Widget {
    texture: Texture,
    position: IVec2,
    pub is_enabled: bool,
}

impl Widget {
    pub fn new(file_name: &str, position: IVec2) -> Self {
        Self {
            texture: Texture::new(file_name),
            position,
            is_enabled: false,
        }
    }

    pub fn bind(&self, shader: &Shader, index: u32) {
        unsafe {
            shader.set_int(format!("u_textures[{}]", index).as_str(), index as i32);
            shader.set_ivec2(format!("u_texture_pos[{}]", index).as_str(), self.position);
            self.texture.bind(gl::TEXTURE0 + index as u32);
        }
    }
}
