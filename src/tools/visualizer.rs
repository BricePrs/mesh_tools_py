use sdl2::{keyboard::Keycode, EventPump};
use ultraviolet::Vec3;

use super::ToolManager;

use crate::app::Action;
use crate::renderer::{Camera};
use crate::controller::{Controller, ControlMode};
use crate::utils::Direction;


pub struct Visualizer {
    camera_controller: Controller,
}

impl ToolManager for Visualizer {

    fn get_controller(&self) -> &Controller { &self.camera_controller }

    fn handle_inputs(&mut self, event_pump: &mut EventPump) -> Vec<Action> {
        let mut action_vec = Vec::new();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => action_vec.push(Action::Quit),

                Event::MouseMotion { xrel, yrel, .. } => {
                    self.camera_controller.mouse_inputs[0] = -xrel as f32;
                    self.camera_controller.mouse_inputs[1] = yrel as f32;
                }

                Event::MouseWheel { y, .. } => {
                    self.camera_controller.mouse_inputs[2] = y as f32;
                }

                Event::KeyDown { keycode, .. } => {
                    match self.handle_key_down(keycode.unwrap()) {
                        Some(a) => action_vec.push(a),
                        None => (),
                    };
                }

                Event::KeyUp { keycode, .. } => {
                    match self.handle_key_up(keycode.unwrap()) {
                        Some(a) => action_vec.push(a),
                        None => (),
                    };
                }

                _ => (),
            };
        }

        action_vec
    }


    fn render_set_up(&mut self, delta_time: f32) {
        self.camera_controller.apply_inputs(delta_time);
    }
}

impl Visualizer {
    pub fn new(w_width: f32, w_height: f32) -> Self {
        let camera = Camera::new(
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 1., 0.),
            0.,
            0.,
            70_f32.to_radians(),
            w_width / w_height,
        );
        Visualizer {
            camera_controller: Controller::new_look_at_anchor(camera, Vec3::zero(), 1., 10.),
            //camera_controller: Controller::new_inertia(),
        }
    }

    pub fn switch_controller(&mut self) {
        match self.camera_controller.get_mode() {
            ControlMode::Absolute(data) => {
                let radius = data.get_scale();
                let camera = self.camera_controller.get_camera();
                let center = camera.get_position().clone() + camera.get_fwd_dir() * radius;
                self.camera_controller = Controller::new_look_at_anchor(camera.clone(), center, radius, 10.);
            }
            ControlMode::LookAtAnchor(data) => {
                self.camera_controller = Controller::new_absolute(
                    self.camera_controller.get_camera().clone(),
                    data.get_radius(),
                );
            }
        }
    }


    fn handle_key_down(&mut self, keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Escape => Some(Action::Quit),
            Keycode::Space => Some(Action::SwapCursorMode),

            Keycode::Z => {
                self.camera_controller.key_inputs[1] = Direction::Positive;
                None
            },
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = Direction::Negative;
                None
            },
            Keycode::S => {
                self.camera_controller.key_inputs[1] = Direction::Negative;
                None
            },
            Keycode::D => {
                self.camera_controller.key_inputs[0] = Direction::Positive;
                None
            },

            _ => None,
        }
    }

    fn handle_key_up(&mut self, keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Z => {
                self.camera_controller.key_inputs[1] = Direction::Null;
                None
            },
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = Direction::Null;
                None
            },
            Keycode::S => {
                self.camera_controller.key_inputs[1] = Direction::Null;
                None
            },
            Keycode::D => {
                self.camera_controller.key_inputs[0] = Direction::Null;
                None
            },

            Keycode::Tab => {
                self.switch_controller();
                None
            }
            _ => None,
        }
    }
}
