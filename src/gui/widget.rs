use ultraviolet::Vec2;
use crate::renderer::texture::Texture;
use gl::types::GLenum;

pub struct Widget {
    texture: Texture,
    position: Vec2,
    is_enabled: bool,
}

impl Widget {
    pub fn new(file_name: &str) -> Self {
        Self {
            texture: Texture::new(file_name),
            position: Vec2::zero(),
            is_enabled: true,
        }
    }

    pub fn bind_texture(&self, texture_index: GLenum) {
        unsafe { self.texture.bind(texture_index); }
    }
}
