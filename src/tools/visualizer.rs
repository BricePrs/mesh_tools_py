use sdl2::{keyboard::Keycode, EventPump};
use crate::app::Action;
use crate::renderer::{Camera, Shader};
use super::ToolManager;


pub struct Visualizer {

}

impl ToolManager for Visualizer {

    fn handle_inputs(&self, event_pump: &mut EventPump) -> Vec<Action> {
        let mut action_vec = Vec::new();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit { .. } => action_vec.push(Action::Quit),

                Event::MouseMotion { xrel, yrel, .. } => {
                    action_vec.push(Action::CameraRotSpeed(-xrel as f32, yrel as f32));
                }

                Event::KeyDown { keycode, .. } => {
                    match Self::handle_key_down(keycode.unwrap()) {
                        Option::Some(a) => action_vec.push(a),
                        Option::None => (),
                    };
                }

                Event::KeyUp { keycode, .. } => {
                    match Self::handle_key_up(keycode.unwrap()) {
                        Option::Some(a) => action_vec.push(a),
                        Option::None => (),
                    };
                }

                _ => (),
            };
        }

        action_vec
    }

    fn use_shader(&self, shader: &Shader, camera: &Camera) {
        shader.use_current();
        let view = camera.get_view_matrix();
        let projection = camera.get_projection_matrix();
        shader.set_mat4("u_view", view);
        shader.set_mat4("u_projection", projection);

    }
}


impl Visualizer {

    pub fn new() -> Self {
        Visualizer {}
    }

    fn handle_key_down(keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Escape => Some(Action::Quit),
            Keycode::Z => Option::Some(Action::CameraMvtSpeed(1., 0.)),
            Keycode::Q => Option::Some(Action::CameraMvtSpeed(0., -1.)),
            Keycode::S => Option::Some(Action::CameraMvtSpeed(-1., 0.)),
            Keycode::D => Option::Some(Action::CameraMvtSpeed(0., 1.)),
            Keycode::Space => Option::Some(Action::SwapCursorMode),

            _ => Option::None,
        }
    }

    fn handle_key_up(keycode: Keycode) -> Option<Action> {
        match keycode {
            Keycode::Z => Option::Some(Action::CameraStop),
            Keycode::Q => Option::Some(Action::CameraStop),
            Keycode::S => Option::Some(Action::CameraStop),
            Keycode::D => Option::Some(Action::CameraStop),
            _ => Option::None,
        }
    }
}