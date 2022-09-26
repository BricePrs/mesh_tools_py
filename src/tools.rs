
mod visualizer;

use sdl2::EventPump;
pub use visualizer::Visualizer;
use crate::app::Action;
use crate::renderer::{Shader, Camera};

pub trait ToolManager {
    fn handle_inputs(&self, event_pump: &mut EventPump) -> Vec<Action>;
    fn use_shader(&self, shader: &Shader, camera: &Camera);
}