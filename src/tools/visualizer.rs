use sdl2::{keyboard::Keycode, EventPump};
use sdl2::mouse::MouseButton;
use ultraviolet::Vec3;

use super::ToolManager;

use crate::app::Action;
use crate::renderer::{Camera};
use crate::controller::{Controller, ControlMode};

pub struct Visualizer {
    camera_controller: Controller,
    mouse_displayed: bool,
    mouse_left_pressed: bool,
    mouse_right_pressed: bool,
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

                    if self.mouse_displayed && self.mouse_left_pressed {
                        self.camera_controller.mouse_inputs[0] = -xrel as f32;
                        self.camera_controller.mouse_inputs[1] = -yrel as f32;
                    }
                    if self.mouse_displayed && self.mouse_right_pressed {
                        self.camera_controller.key_inputs[0] = xrel as f32;
                        self.camera_controller.key_inputs[1] = -yrel as f32;
                    }

                    if self.mouse_displayed {
                        continue;
                    }

                    self.camera_controller.mouse_inputs[0] = -xrel as f32;
                    self.camera_controller.mouse_inputs[1] = yrel as f32;
                }

                Event::MouseWheel { y, .. } => {
                    self.camera_controller.mouse_inputs[2] = y as f32;
                }

                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => self.mouse_left_pressed = true,
                        MouseButton::Right => self.mouse_right_pressed = true,
                        _ => (),
                    }
                }

                Event::MouseButtonUp { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => self.mouse_left_pressed = false,
                        MouseButton::Right => self.mouse_right_pressed = false,
                        _ => (),
                    }
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
        if self.mouse_displayed {
            self.camera_controller.key_inputs = [0., 0.];
        }
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
            camera_controller: Controller::new_look_at_anchor(camera, Vec3::zero(), 20., 10.),
            mouse_displayed: true,
            mouse_left_pressed: false,
            mouse_right_pressed: false,
        }
    }

    pub fn switch_controller(&mut self) {
        match self.camera_controller.get_mode() {
            ControlMode::Absolute(data) => {
                let radius = self.camera_controller.get_scale();
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

            Keycode::Z => {
                self.camera_controller.key_inputs[1] = 1.;
                None
            },
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = -1.;
                None
            },
            Keycode::S => {
                self.camera_controller.key_inputs[1] = -1.;
                None
            },
            Keycode::D => {
                self.camera_controller.key_inputs[0] = 1.;
                None
            },

            _ => None,
        }
    }

    fn handle_key_up(&mut self, keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Z => {
                self.camera_controller.key_inputs[1] = 0.;
                None
            },
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = 0.;
                None
            },
            Keycode::S => {
                self.camera_controller.key_inputs[1] = 0.;
                None
            },
            Keycode::D => {
                self.camera_controller.key_inputs[0] = 0.;
                None
            },

            Keycode::Tab => {
                self.mouse_displayed = !self.mouse_displayed;
                self.switch_controller();
                Some(Action::SwapCursorMode)
            }
            _ => None,
        }
    }
}
