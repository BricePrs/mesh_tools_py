use crate::{tools};
use crate::renderer;
use crate::renderer::geometry::mesh;
use ultraviolet::Vec3;

use sdl2::video::GLProfile;
use crate::renderer::{Scene, BatchType};
use crate::tools::ToolManager;

pub enum Action {
    Quit,
    CameraStop,
    CameraMvtSpeed(f32, f32),
    CameraRotSpeed(f32, f32),
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

    let mut camera = renderer::Camera::new(
        Vec3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        0.,
        0.,
        70_f32.to_radians(),
        w_width as f32 / w_height as f32,
);

    let tool_manager = tools::Visualizer::new();

    let mut scene = Scene::new();
    scene.add(BatchType::Default, mesh::cube::new());
    scene.add(BatchType::Anchor, mesh::plane_grid::new());
    scene.add(BatchType::Anchor, mesh::axis3d::new());

    unsafe {
        gl::ClearColor(0.05, 0.05, 0.05, 1.);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let mut camera_speed_fwd = 0.;
    let mut camera_speed_rgt = 0.;

    'main_loop: loop {
        // handle events this frame

        let mut camera_is_moving = false;
        for action in tool_manager.handle_inputs(&mut event_pump) {
            match action {
                Action::Quit => break 'main_loop,
                Action::SwapCursorMode => {
                    is_cursor_displayed = !is_cursor_displayed;
                    sdl_context
                        .mouse()
                        .set_relative_mouse_mode(!is_cursor_displayed);
                }
                Action::CameraStop => {
                    if !camera_is_moving {
                        camera_speed_fwd = 0.;
                        camera_speed_rgt = 0.;
                    }
                }
                Action::CameraMvtSpeed(x, y) => {
                    camera_speed_fwd = x;
                    camera_speed_rgt = y;
                    camera_is_moving = true;
                }
                Action::CameraRotSpeed(x, y) => camera.rotate(x, y),
            }
        }

        camera.translate(camera_speed_fwd, camera_speed_rgt);

        // now the events are clear
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            renderer::draw_scene(&tool_manager, &camera, &scene);
        }
        // here's where we could change the world state and draw.
        win.gl_swap_window();
    }
}
