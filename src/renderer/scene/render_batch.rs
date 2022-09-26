use crate::renderer::geometry::mesh::Mesh;
use crate::renderer::Shader;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BatchType {
    Default,
    Anchor,
}

pub(crate) static BATCH_TYPES: [BatchType; 2] = [
    BatchType::Default,
    BatchType::Anchor,
];

pub struct RenderBatch {
    pub objects: Vec<Mesh>,
    shader: Shader,
}

impl RenderBatch {

    pub fn map(batch_type: &BatchType) -> Self {
        let shader = match batch_type {
            BatchType::Default => Shader::new("default_colored.vsh", "default_colored.fsh"),
            BatchType::Anchor => Shader::new("default_colored_gizmos.vsh", "default_colored_gizmos.fsh"),
        };
        Self {
            shader,
            objects: Vec::new()
        }
    }

    pub fn push(&mut self, object: Mesh) {
        self.objects.push(object)
    }

    pub fn get_shader(&self) -> &Shader {
        &self.shader
    }
}
