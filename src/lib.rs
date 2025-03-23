pub extern crate nalgebra_glm as glm;

pub mod application;
pub mod hlgl;

pub use application::*;
use gl::types::GLsizei;
use sdl3::EventPump;
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;
use sdl3::video::{GLContext, Window};

pub struct Engine {
    window: Window,
    gl_ctx: GLContext,
    event_pump: EventPump,
}

pub fn init() -> Engine {
    colog::init();

    let sdl_context = sdl3::init().unwrap();
    log::debug!("Initialized SDL3 v{}", sdl3::version::version());

    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("test", 800, 600)
        .high_pixel_density()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    log::debug!(
        "Opened window (size: {:?}, pixel size: {:?})",
        window.size(),
        window.size_in_pixels()
    );

    let ctx = window.gl_create_context().unwrap();
    window.gl_make_current(&ctx).unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name).unwrap() as *const _);

    log::debug!(
        "OpenGL v{}.{}",
        gl_attr.context_version().0,
        gl_attr.context_version().1
    );

    Engine {
        window,
        gl_ctx: ctx,
        event_pump: sdl_context.event_pump().unwrap(),
    }
}

pub fn run_app<T: Application>(engine: &mut Engine, app: &mut T) {
    unsafe {
        gl::Viewport(
            0,
            0,
            engine.window.size_in_pixels().0 as GLsizei,
            engine.window.size_in_pixels().1 as GLsizei,
        );
    }

    'running: loop {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        app.update(0.0);
        app.draw();

        engine.window.gl_swap_window();

        for event in engine.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: sdl3::mouse::MouseButton::Left,
                    ..
                } => app.mouse_button_event(x, y, MouseButton::Left, MouseAction::Press),
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: sdl3::mouse::MouseButton::Right,
                    ..
                } => app.mouse_button_event(x, y, MouseButton::Right, MouseAction::Press),

                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        app.window_resized(width as u32, height as u32);
                    }
                    WindowEvent::PixelSizeChanged(width, height) => unsafe {
                        app.window_pixel_resized(width as u32, height as u32);
                    },
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
