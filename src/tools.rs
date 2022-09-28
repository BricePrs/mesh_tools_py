mod visualizer;

use crate::app::Action;
use crate::renderer::{Camera, Shader};
use sdl2::EventPump;
pub use visualizer::Visualizer;
use crate::controller::Controller;

pub trait ToolManager {
    fn get_controller(&self) -> &Controller;
    fn handle_inputs(&mut self, event_pump: &mut EventPump) -> Vec<Action>;
    fn render_set_up(&mut self, delta_time: f32);
}
