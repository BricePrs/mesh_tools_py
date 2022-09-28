use crate::{interface, renderer};
use crate::renderer::geometry::mesh;
use crate::tools;

use crate::renderer::{BatchType, Scene};
use crate::tools::ToolManager;
use sdl2::video::GLProfile;

pub enum Action {
    Quit,
    SwapCursorMode,
}

pub fn create_window(w_width: u32, w_height: u32) {
    let mut is_cursor_displayed: bool = false;

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

    video_subsystem
        .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
        .unwrap();
    sdl_context
        .mouse()
        .set_relative_mouse_mode(!is_cursor_displayed);

    let mut tool_manager = tools::Visualizer::new(w_width as f32, w_height as f32);

    let mut scene = Scene::new();

    scene.add(BatchType::Default, interface::file::load_mesh("HumanHead.ply"));
    //scene.add(BatchType::Default, mesh::cube::new());

    scene.add(BatchType::Anchor, mesh::plane_grid::new());
    scene.add(BatchType::Anchor, mesh::axis3d::new());

    unsafe {
        gl::ClearColor(0.25, 0.25, 0.25, 1.);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut frame_time = std::time::Instant::now();

    'main_loop: loop {

        let delta_time = frame_time.elapsed().as_secs_f32();
        frame_time = std::time::Instant::now();
        println!("{}Hz",1./delta_time);

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
        }

        win.gl_swap_window();

    }
}
