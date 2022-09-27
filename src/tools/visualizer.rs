use sdl2::{keyboard::Keycode, EventPump};

use super::ToolManager;

use crate::app::Action;
use crate::renderer::{Camera};
use crate::controller::Controller;
use crate::utils::Direction;


pub struct Visualizer {
    camera_controller: Controller,
}

impl ToolManager for Visualizer {

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


    fn render_set_up(&mut self, camera: &mut Camera, delta_time: f32) {
        self.camera_controller.apply_inputs(camera, delta_time);
    }
}

impl Visualizer {
    pub fn new() -> Self {
        Visualizer {
            camera_controller: Controller::new_look_at_anchor(),
            //camera_controller: Controller::new_inertia(),
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
            _ => None,
        }
    }
}
