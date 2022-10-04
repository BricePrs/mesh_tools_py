use super::ControllerData;
use crate::renderer::Camera;
use ultraviolet::Vec3;

pub struct LookAtAnchorData {
    center: Vec3,
    radius: f32,
    speed: f32,
}

impl LookAtAnchorData {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self {
            center,
            radius,
            speed: 2.,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}

impl ControllerData for LookAtAnchorData {
    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 3]) {
        self.center +=
            (-camera.get_rgt_dir() * axis_key[0] + camera.get_up_dir() * axis_key[1]) * self.speed;
        self.radius *= 1. + 0.1 * sign(axis_mouse[2]);
        camera.rotate(axis_mouse[0], -axis_mouse[1]);
        camera.project_onto_sphere(self.center, self.radius);
    }
}

pub fn sign(a: f32) -> f32 {
    if a < 0. {
        return -1.;
    } else if a > 0. {
        return 1.;
    }
    0.
}
