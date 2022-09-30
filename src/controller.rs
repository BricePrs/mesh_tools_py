mod absolute_controller;
mod look_at_anchor_controller;

use ultraviolet::Vec3;
use absolute_controller::AbsoluteData;
use look_at_anchor_controller::LookAtAnchorData;

use crate::renderer::Camera;


pub enum ControlMode {
    Absolute(AbsoluteData),
    LookAtAnchor(LookAtAnchorData),
}

pub struct Controller {
    mode: ControlMode,
    camera: Camera,
    scale: f32,
    pub(crate) key_inputs: [f32; 2],
    pub(crate) mouse_inputs: [f32; 3],
}

trait ControllerData {
    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 3]);
}

impl Controller {

    pub fn new_absolute(camera: Camera, scale: f32) -> Self {
        Self {
            camera,
            mode: ControlMode::Absolute(AbsoluteData::new()),
            scale,
            key_inputs: [0., 0.],
            mouse_inputs: [0., 0., 0.],
        }
    }

    pub fn new_look_at_anchor(camera: Camera, center: Vec3, radius: f32, scale: f32) -> Self {
        Self {
            camera,
            mode: ControlMode::LookAtAnchor(LookAtAnchorData::new(center, radius)),
            scale,
            key_inputs: [0., 0.],
            mouse_inputs: [0., 0., 0.],
        }
    }

    pub fn get_scale(&self) -> f32 { self.scale }
    pub fn get_camera(&self) -> &Camera { &self.camera }
    pub fn get_mode(&self) -> &ControlMode { &self.mode }

    pub fn apply_inputs(&mut self, delta_time: f32) {
        match &mut self.mode {
            ControlMode::Absolute(data) => {
                data.apply_inputs(
                    &mut self.camera,
                    &self.key_inputs.map(|x| x*delta_time*self.scale),
                    &self.mouse_inputs.map(|x| x*delta_time),
                );
            },
            ControlMode::LookAtAnchor(data) => data.apply_inputs(
                &mut self.camera,
                &self.key_inputs.map(|x| x*delta_time*self.scale),
                &self.mouse_inputs.map(|x| x*delta_time),
            ),
        };
        self.mouse_inputs = [0.; 3];
    }

}
