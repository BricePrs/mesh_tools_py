
use std::io::Write;

mod shader;
mod camera;

pub mod geom_mngr;

pub use shader::Shader;
pub use camera::Camera;

pub fn clear_error() {
    unsafe {
        while gl::GetError() != gl::NO_ERROR {};
    }
}

pub fn get_error() {
    let mut error = false;
    unsafe {
        loop {
            match gl::GetError() {

                gl::NO_ERROR => {
                    match error {
                        true => {        
                            std::io::stdout().flush().unwrap();
                            panic!();
                        },
                        false => return,
                    }
                },

                a => {
                    println!("Error : {}", a);
                    error = true;
                }
            }
        };

    }
}

