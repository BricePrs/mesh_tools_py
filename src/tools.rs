mod visualizer;

use crate::app::Action;
use crate::renderer::{Camera, Shader};
use sdl2::EventPump;
pub use visualizer::Visualizer;

pub trait ToolManager {
    fn handle_inputs(&mut self, event_pump: &mut EventPump) -> Vec<Action>;
    fn render_set_up(&mut self, scene: &mut Camera, delta_time: f32);
}
