use ultraviolet::{projection::rh_yup::perspective_gl, Mat4, Vec3};

pub use crate::controller::Controller;

#[derive(Copy, Clone)]
pub struct Camera {
    position: Vec3,

    fwd_dir: Vec3,
    rgt_dir: Vec3,
    up_dir: Vec3,

    world_up: Vec3,

    pitch: f32,
    yaw: f32,

    rot_speed: f32,
    mvt_speed: f32,

    fov: f32,
    aspect: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        world_up: Vec3,
        pitch: f32,
        yaw: f32,
        fov: f32,
        aspect: f32,
    ) -> Self {
        let mut incomplete_camera = Self {
            position,

            fwd_dir: Vec3::zero(),
            rgt_dir: Vec3::zero(),
            up_dir: Vec3::zero(),

            world_up,

            pitch,
            yaw,

            rot_speed: 5.0_f32,
            mvt_speed: 20.0_f32,

            fov,
            aspect,
        };
        incomplete_camera.update_camera_base();
        incomplete_camera
    }

    pub fn get_position(&self) -> &Vec3 {
        &self.position
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::from_euler_angles(0., -self.pitch, -self.yaw).transposed()
            * Mat4::from_translation(self.position)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        perspective_gl(self.fov, self.aspect, 0.1_f32, 100.0_f32)
    }

    pub fn translate(&mut self, delta_x: f32, delta_y: f32) {
        self.position += (self.fwd_dir * delta_x + self.rgt_dir * delta_y) * self.mvt_speed;
        self.update_camera_base();
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += self.rot_speed * delta_x;
        self.pitch += self.rot_speed * delta_y;

        // Check if in range (+-PI/2)
        if self.pitch > 1.53 {
            self.pitch = 1.53;
        }
        if self.pitch < -1.53 {
            self.pitch = -1.53;
        }
        self.update_camera_base();
    }

    pub fn project_onto_sphere(&mut self, center: Vec3, radius: f32) {
        self.position = -self.fwd_dir * radius + center;
    }

    fn update_camera_base(&mut self) {
        self.fwd_dir = Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.cos(),
        );
        self.rgt_dir = self.fwd_dir.cross(self.world_up).normalized();
        self.up_dir = self.rgt_dir.cross(self.fwd_dir).normalized();
    }

    pub fn get_fwd_dir(&self) -> Vec3 {
        self.fwd_dir
    }
    pub fn get_rgt_dir(&self) -> Vec3 {
        self.rgt_dir
    }
    pub fn get_up_dir(&self) -> Vec3 {
        self.up_dir
    }
}
