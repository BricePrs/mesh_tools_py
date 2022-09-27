mod absolute_controller;
mod look_at_anchor_controller;

use absolute_controller::AbsoluteData;
use look_at_anchor_controller::LookAtAnchorData;

use crate::utils::Direction;
use crate::renderer::Camera;


enum ControlMode {
    Absolute(AbsoluteData),
    LookAtAnchor(LookAtAnchorData),
}


pub struct Controller {
    mode: ControlMode,
    pub(crate) key_inputs: [Direction; 2],
    pub(crate) mouse_inputs: [f32; 2],
}

trait ControllerData {
    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 2]);
}

impl Controller {

    pub fn new_inertia() -> Self {
        Self {
            mode: ControlMode::Absolute(AbsoluteData::new()),
            key_inputs: [Direction::Null, Direction::Null],
            mouse_inputs: [0., 0.],
        }
    }

    pub fn new_look_at_anchor() -> Self {
        Self {
            mode: ControlMode::LookAtAnchor(LookAtAnchorData::new()),
            key_inputs: [Direction::Null, Direction::Null],
            mouse_inputs: [0., 0.],
        }
    }

    pub fn apply_inputs(&mut self, camera: &mut Camera, delta_time: f32) {
        match &mut self.mode {
            ControlMode::Absolute(data) => {
                data.apply_inputs(
                    camera,
                    &Self::axis_to_f32(&self.key_inputs, delta_time),
                    &self.mouse_inputs.map(|x| x*delta_time),
                )
            },
            ControlMode::LookAtAnchor(data) => data.apply_inputs(
                camera,
                &Self::axis_to_f32(&self.key_inputs, delta_time),
                &self.mouse_inputs.map(|x| x*delta_time),
            ),
        };
        self.mouse_inputs = [0.; 2];
    }

    fn axis_to_f32(axis: &[Direction; 2], factor: f32) -> [f32; 2] {
        axis.clone().map(|x| match x {
            Direction::Negative => -factor,
            Direction::Null => 0.,
            Direction::Positive => factor,
        })
    }

}
