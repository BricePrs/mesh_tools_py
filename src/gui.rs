use ultraviolet::IVec2;
use crate::renderer::geometry::mesh::Mesh;
use crate::renderer::Shader;

mod widget;

use widget::Widget;

pub struct Gui {
    resolution: IVec2,
    surface: Mesh,
    shader: Shader,
    widgets: Vec<Widget>,
}

impl Gui {
    pub fn new(w_width: u32, w_height: u32) -> Self {

        let resolution = IVec2::new(w_width as i32, w_height as i32);

        let vertices = vec![
            [-1., -1., 0., 0., 0., 0.],
            [1.0, -1., 0., 0., 0., 0.],
            [1.0, 1.0, 0., 0., 0., 0.],
            [-1., 1.0, 0., 0., 0., 0.],
        ];
        let indices = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        Self {
            resolution,
            surface: Mesh::new(vertices, indices),
            shader: Shader::new("default_gui.vsh", "default_gui.fsh"),
            widgets: Vec::new(),
        }
    }

    pub unsafe fn render(&self) {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        self.shader.use_current();
        let mut texture_index = gl::TEXTURE0;
        for widget in &self.widgets {
            let texture = widget.bind_texture(texture_index);
            texture_index += 1;
        }
        self.surface.draw();
    }
}
