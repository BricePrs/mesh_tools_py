use ultraviolet::Vec3;
use crate::renderer::Camera;
use super::ControllerData;


pub struct LookAtAnchorData {
    center: Vec3,
    radius: f32,
}

impl LookAtAnchorData {
    pub fn new() -> Self {
        Self { center: Vec3::new(0., 0., -6.), radius: 8. }
    }
}

impl ControllerData for LookAtAnchorData {

    fn apply_inputs(&mut self, camera: &mut Camera, axis_key: &[f32; 2], axis_mouse: &[f32; 2]) {
        camera.rotate(-axis_mouse[0],-axis_mouse[1]);
        camera.project_onto_sphere(self.center, self.radius);
    }

}
