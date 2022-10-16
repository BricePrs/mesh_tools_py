use crate::renderer;
use crate::renderer::geometry::mesh;
use crate::tools;
use std::io::{stdout, Write};

use crate::gui::Gui;
use crate::renderer::{BatchType, Scene};
use crate::tools::ToolManager;
use sdl2::video::GLProfile;

pub enum Action {
    Quit,
    SwapCursorMode,
}

pub fn create_window(w_width: u32, w_height: u32) {
    let mut is_cursor_displayed: bool = true;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Don't use deprecated OpenGL functions
    gl_attr.set_context_profile(GLProfile::Core);

    // Set the context into debug mode
    gl_attr.set_context_flags().debug().set();

    gl_attr.set_context_version(3, 3);

    // Enable anti-aliasing
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let win = video_subsystem
        .window("Mesh tools", w_width, w_height)
        .opengl()
        .build()
        .unwrap();

    assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    assert_eq!(gl_attr.context_version(), (3, 3));

    /* create a new OpenGL context and make it current */
    let gl_context = win.gl_create_context().unwrap();
    win.gl_make_current(&gl_context).unwrap();

    gl::load_with(|f_name| video_subsystem.gl_get_proc_address(f_name) as *const _);

    sdl_context
        .mouse()
        .set_relative_mouse_mode(!is_cursor_displayed);

    let mut tool_manager = tools::Visualizer::new(w_width as f32, w_height as f32);

    let gui = Gui::new(w_width, w_height);

    let mut scene = Scene::new();

    scene.add(BatchType::Default, crate::interface::file::load_mesh("flavieux.ply", 100.));
    //scene.add(BatchType::Default, mesh::colored_cube::new());

    scene.add(BatchType::Anchor, mesh::plane_grid::new());
    scene.add(BatchType::Anchor, mesh::axis3d::new());

    unsafe {
        gl::ClearColor(0.25, 0.25, 0.25, 1.);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::DepthFunc(gl::LESS);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut frame_time = std::time::Instant::now();

    let mut mean_delta_time = 0.;
    let mut frame_count = 0;

    'main_loop: loop {
        let delta_time = frame_time.elapsed().as_secs_f32();
        frame_time = std::time::Instant::now();
        frame_count += 1;
        mean_delta_time += delta_time;
        if frame_count > 100 {
            print!("\r{}Hz", frame_count as f32 / mean_delta_time);
            frame_count = 0;
            mean_delta_time = 0.;
            stdout().flush().expect("Stdout flush error");
        }

        for action in tool_manager.handle_inputs(&mut event_pump) {
            match action {
                Action::Quit => break 'main_loop,

                Action::SwapCursorMode => {
                    is_cursor_displayed = !is_cursor_displayed;
                    sdl_context
                        .mouse()
                        .set_relative_mouse_mode(!is_cursor_displayed);
                }
            }
        }

        tool_manager.render_set_up(delta_time);

        unsafe {
            renderer::draw_scene(&scene, tool_manager.get_controller().get_camera());
            //gui.render();
        }

        win.gl_swap_window();
    }
}
