use crate::controller::look_at_anchor_controller::LookAtAnchorData;
use crate::renderer::Camera;
use super::ControllerData;

pub struct AbsoluteData {
    speed: f32,
}

impl AbsoluteData {
    pub fn new() -> Self { Self{ speed: 0.2 } }

    pub fn get_scale(&self) {}
}

impl ControllerData for AbsoluteData {

    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 3]) {
        camera.translate(axis_key[1]*self.speed, axis_key[0]*self.speed);
        camera.rotate(axis_mouse[0],axis_mouse[1]);
    }

}
