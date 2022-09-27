use crate::renderer::Camera;
use super::ControllerData;


pub struct AbsoluteData {}

impl AbsoluteData {
    pub fn new() -> Self {
        Self {}
    }
}

impl ControllerData for AbsoluteData {

    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 2]) {
        camera.translate(axis_key[1], axis_key[0]);
        camera.rotate(axis_mouse[0],axis_mouse[1]);
    }

}
