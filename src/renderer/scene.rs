pub(crate) mod render_batch;

use crate::renderer::geometry::mesh::Mesh;
use render_batch::{RenderBatch, BATCH_TYPES};
use std::collections::HashMap;
use ultraviolet::Vec3;

use crate::renderer::{camera, Camera};

pub use render_batch::BatchType;

pub struct Scene {
    camera: Camera,
    render_queue: HashMap<BatchType, RenderBatch>,
}

impl Scene {
    pub fn new(w_width: f32, w_height: f32) -> Self {
        let mut render_queue = HashMap::new();
        for batch_type in &BATCH_TYPES {
            render_queue.insert(batch_type.clone(), RenderBatch::map(batch_type));
        }
        let camera = Camera::new(
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 1., 0.),
            0.,
            0.,
            70_f32.to_radians(),
            w_width / w_height,
        );
        Self {
            camera,
            render_queue,
        }
    }

    pub fn add(&mut self, batch_type: BatchType, object: Mesh) {
        self.render_queue.get_mut(&batch_type).unwrap().push(object);
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub unsafe fn draw(&self) {
        let view = self.camera.get_view_matrix();
        let projection = self.camera.get_projection_matrix();

        for batch_type in &BATCH_TYPES {
            let render_batch = self.render_queue.get(batch_type).unwrap();

            let shader = render_batch.get_shader(); // ref not mut but using set

            shader.use_current();

            shader.set_mat4("u_view", &view);
            shader.set_mat4("u_projection", &projection);

            shader.set_vec3("u_camPosition", &self.camera.get_position());

            for object in &render_batch.objects {
                object.draw();
            }
        }
    }

}
