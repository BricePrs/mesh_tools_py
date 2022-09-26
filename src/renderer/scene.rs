pub(crate) mod render_batch;

use std::collections::HashMap;
use crate::renderer::geometry::mesh::Mesh;
use render_batch::{RenderBatch, BATCH_TYPES};

pub use render_batch::BatchType;
use crate::renderer::Camera;
use crate::tools::ToolManager;

pub struct Scene {
    render_queue:HashMap<BatchType, RenderBatch>,
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
        self.render_queue.get_mut(&batch_type)
            .unwrap()
            .push(object);
    }

    pub unsafe fn draw(&self, tool_manager: &dyn ToolManager, camera: &Camera) {
        for batch_type in &BATCH_TYPES {
            let render_batch = self.render_queue.get(batch_type).unwrap();
            tool_manager.use_shader(render_batch.get_shader(), &camera);

            for object in &render_batch.objects {
                object.draw();
            }
        }
    }
}
