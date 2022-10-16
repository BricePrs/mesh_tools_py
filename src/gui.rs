use crate::renderer::geometry::mesh::Mesh;
use crate::renderer::Shader;
use ultraviolet::IVec2;

mod widget;

pub use widget::Widget;

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
        let indices = vec![0, 1, 2, 0, 2, 3];
        Self {
            resolution,
            surface: Mesh::new(vertices, indices),
            shader: Shader::new("default_gui.vsh", "default_gui.fsh"),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Widget) -> usize {
        self.widgets.push(widget);
        self.widgets.len()
    }

    pub unsafe fn render(&self) {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        self.shader.use_current();
        self.shader.set_ivec2("u_resolution", self.resolution);
        let mut index = 0;
        for widget in &self.widgets {
            if widget.is_enabled {
                widget.bind(&self.shader, index);
                index += 1;
            }
        }
        self.surface.draw();
        for j in 0..=index {
            gl::ActiveTexture(gl::TEXTURE0+j);
            unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}
        }
    }
}
