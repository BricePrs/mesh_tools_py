mod absolute_controller;
mod look_at_anchor_controller;

use ultraviolet::Vec3;
use absolute_controller::AbsoluteData;
use look_at_anchor_controller::LookAtAnchorData;

use crate::utils::Direction;
use crate::renderer::Camera;


pub enum ControlMode {
    Absolute(AbsoluteData),
    LookAtAnchor(LookAtAnchorData),
}


pub struct Controller {
    mode: ControlMode,
    camera: Camera,
    pub(crate) key_inputs: [Direction; 2],
    pub(crate) mouse_inputs: [f32; 3],
}

trait ControllerData {
    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 3]);
}

impl Controller {

    pub fn new_absolute(camera: Camera, scale: f32) -> Self {
        Self {
            camera,
            mode: ControlMode::Absolute(AbsoluteData::new(scale)),
            key_inputs: [Direction::Null, Direction::Null],
            mouse_inputs: [0., 0., 0.],
        }
    }

    pub fn new_look_at_anchor(camera: Camera, center: Vec3, radius: f32, speed: f32) -> Self {
        Self {
            camera,
            mode: ControlMode::LookAtAnchor(LookAtAnchorData::new(center, radius, speed)),
            key_inputs: [Direction::Null, Direction::Null],
            mouse_inputs: [0., 0., 0.],
        }
    }

    pub fn get_camera(&self) -> &Camera { &self.camera }
    pub fn get_mode(&self) -> &ControlMode { &self.mode }

    pub fn apply_inputs(&mut self, delta_time: f32) {
        match &mut self.mode {
            ControlMode::Absolute(data) => {
                data.apply_inputs(
                    &mut self.camera,
                    &Self::axis_to_f32(&self.key_inputs, delta_time),
                    &self.mouse_inputs.map(|x| x*delta_time),
                )
            },
            ControlMode::LookAtAnchor(data) => data.apply_inputs(
                &mut self.camera,
                &Self::axis_to_f32(&self.key_inputs, delta_time),
                &self.mouse_inputs.map(|x| x*delta_time),
            ),
        };
        self.mouse_inputs = [0.; 3];
    }

    fn axis_to_f32(axis: &[Direction; 2], factor: f32) -> [f32; 2] {
        axis.clone().map(|x| match x {
            Direction::Negative => -factor,
            Direction::Null => 0.,
            Direction::Positive => factor,
        })
    }

}
