use crate::controller::look_at_anchor_controller::LookAtAnchorData;
use crate::renderer::Camera;
use super::ControllerData;

pub struct AbsoluteData {
    scale: f32,
}

impl AbsoluteData {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
    pub fn get_scale(&self) -> f32 { self.scale }
}

impl ControllerData for AbsoluteData {

    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 3]) {
        camera.translate(axis_key[1], axis_key[0]);
        camera.rotate(axis_mouse[0],axis_mouse[1]);
    }

}
