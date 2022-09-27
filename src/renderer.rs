use std::io::Write;

mod camera;
mod shader;

pub mod geometry;
mod scene;

pub use camera::Camera;
pub use scene::render_batch::BatchType;
pub use scene::Scene;
pub use shader::Shader;

/// Erase previous draw
/// Render the scene passed as an arg
/// Passed shader must be in use
pub unsafe fn draw_scene(scene: &Scene) {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    scene.draw();
}

pub fn clear_error() {
    unsafe { while gl::GetError() != gl::NO_ERROR {} }
}

pub fn get_error() {
    let mut error = false;
    unsafe {
        loop {
            match gl::GetError() {
                gl::NO_ERROR => match error {
                    true => {
                        std::io::stdout().flush().unwrap();
                        panic!();
                    }
                    false => return,
                },

                a => {
                    println!("Error : {}", a);
                    error = true;
                }
            }
        }
    }
}
