use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use crate::app::Action;

enum InputTimeMode {
    Continuous,
    Discrete,
}

struct KeysHolder {
    keys_down: Vec<Keycode>,
    keys_up: Vec<Keycode>,
}

enum InputAxisEntry {
    Keyboard(KeysHolder),
    WASD(InputTimeMode),
    Mouse(InputTimeMode),
}

struct AxisParam {
    field_nbr: usize,
    input_axis_entry: InputAxisEntry
}

pub struct InputManager {
    axes: Vec<f32>,
    axes_param: Vec<AxisParam>,
}

impl InputManager {

    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            axes_param: Vec::new(),
        }
    }

    fn handle_inputs(&mut self) {
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

                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Left => self.mouse_left_pressed = true,
                    MouseButton::Right => self.mouse_right_pressed = true,
                    _ => (),
                },

                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Left => self.mouse_left_pressed = false,
                    MouseButton::Right => self.mouse_right_pressed = false,
                    _ => (),
                },

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

    }

    fn handle_key_down(&mut self, keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Escape => Some(Action::Quit),
            Keycode::Space => {
                None
            },
            Keycode::Z => {
                self.camera_controller.key_inputs[1] = 1.;
                None
            }
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = -1.;
                None
            }
            Keycode::S => {
                self.camera_controller.key_inputs[1] = -1.;
                None
            }
            Keycode::D => {
                self.camera_controller.key_inputs[0] = 1.;
                None
            }

            _ => None,
        }
    }

    fn handle_key_up(&mut self, keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Z => {
                self.camera_controller.key_inputs[1] = 0.;
                None
            }
            Keycode::Q => {
                self.camera_controller.key_inputs[0] = 0.;
                None
            }
            Keycode::S => {
                self.camera_controller.key_inputs[1] = 0.;
                None
            }
            Keycode::D => {
                self.camera_controller.key_inputs[0] = 0.;
                None
            }

            Keycode::Tab => {
                self.mouse_displayed = !self.mouse_displayed;
                self.switch_controller();
                Some(Action::SwapCursorMode)
            }
            _ => None,
        }
    }
}
