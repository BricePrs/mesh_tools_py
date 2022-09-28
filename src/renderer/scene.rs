pub(crate) mod render_batch;

use crate::renderer::geometry::mesh::Mesh;
use render_batch::{RenderBatch, BATCH_TYPES};
use std::collections::HashMap;


pub use render_batch::BatchType;
use crate::renderer::Camera;

pub struct Scene {
    render_queue: HashMap<BatchType, RenderBatch>,
}

impl Scene {
    pub fn new() -> Self {
        let mut render_queue = HashMap::new();
        for batch_type in &BATCH_TYPES {
            render_queue.insert(batch_type.clone(), RenderBatch::map(batch_type));
        }

        Self {
            render_queue,
        }
    }

    pub fn add(&mut self, batch_type: BatchType, object: Mesh) {
        self.render_queue.get_mut(&batch_type).unwrap().push(object);
    }

    pub unsafe fn draw(&self, camera: &Camera) {
        let view = camera.get_view_matrix();
        let projection = camera.get_projection_matrix();

        for batch_type in &BATCH_TYPES {
            let render_batch = self.render_queue.get(batch_type).unwrap();

            let shader = render_batch.get_shader(); // ref not mut but using set

            shader.use_current();

            shader.set_mat4("u_view", &view);
            shader.set_mat4("u_projection", &projection);

            shader.set_vec3("u_camPosition", camera.get_position());

            for object in &render_batch.objects {
                object.draw();
            }
        }
    }

}
