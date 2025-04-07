pub extern crate nalgebra_glm as glm;

pub mod application;
mod averagers;
pub mod gfx;
pub mod hlgl;
pub mod input;
mod time;

mod gl {
    #![allow(unsafe_op_in_unsafe_fn)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use application::*;
pub use averagers::*;
pub use gfx::{Hsla, Hsva, Rgba};
use sdl3::EventPump;
pub use time::*;

pub struct Engine {
    pub g2d: gfx::G2d,
    pub frame_counter: FrameCounter,

    running: bool,

    #[allow(dead_code)]
    ctx: sdl3::video::GLContext,
    pub window: sdl3::video::Window,
    sdl: sdl3::Sdl,
}

impl Engine {
    pub fn window_ortho_projection(&self) -> glm::Mat4 {
        glm::ortho_lh_zo(
            0.0,
            self.window.size().0 as f32,
            self.window.size().1 as f32,
            0.0,
            -1.0,
            1.0,
        )
    }

    pub fn shutdown(&mut self) {
        self.running = false;
    }

    pub fn vsync(&mut self) -> bool {
        self.window
            .subsystem()
            .gl_get_swap_interval()
            .map(|interval| interval == sdl3::video::SwapInterval::VSync)
            .unwrap_or(false)
    }

    pub fn set_vsync(&mut self, enabled: bool) {
        let _ = self.window.subsystem().gl_set_swap_interval(if enabled {
            sdl3::video::SwapInterval::VSync
        } else {
            sdl3::video::SwapInterval::Immediate
        });
    }
}

pub fn init<T>(
    title: &str,
    width: u32,
    height: u32,
    window_build_fn: T,
) -> Result<Engine, Box<dyn std::error::Error>>
where
    T: FnOnce(&mut sdl3::video::WindowBuilder) -> &mut sdl3::video::WindowBuilder,
{
    colog::init();

    let sdl_context = sdl3::init()?;
    log::debug!("Initialized SDL3 v{}", sdl3::version::version());

    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    #[cfg(debug_assertions)]
    {
        gl_attr.set_context_flags().debug().set();
    }
    // gl_attr.set_multisample_buffers(1);
    // gl_attr.set_multisample_samples(4);

    let mut builder = video_subsystem.window(title, width, height);
    let window = window_build_fn(&mut builder)
        .high_pixel_density()
        .opengl()
        .build()?;
    log::debug!(
        "Opened window (size: {:?}, pixel size: {:?}, display_scale: {})",
        window.size(),
        window.size_in_pixels(),
        window.display_scale(),
    );

    let ctx = window.gl_create_context()?;
    window.gl_make_current(&ctx)?;

    gl::load_with(|name| match video_subsystem.gl_get_proc_address(name) {
        None => std::ptr::null(),
        Some(addr) => addr as *const _,
    });

    #[cfg(debug_assertions)]
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(gl_debug_callback), std::ptr::null());
    }

    unsafe {
        gl::PointSize(window.display_scale());
        gl::LineWidth(window.display_scale());
    }

    log::debug!(
        "Loaded OpenGL v{}.{}",
        gl_attr.context_version().0,
        gl_attr.context_version().1
    );

    Ok(Engine {
        window,
        g2d: gfx::G2d::new(gl_attr.context_version()),
        frame_counter: FrameCounter::default(),
        sdl: sdl_context,
        ctx,
        running: false,
    })
}

pub fn run_app<T: Application>(engine: &mut Engine, app: &mut T) {
    engine.running = true;

    // Make sure we get a good value when we ask about vsync later
    engine.set_vsync(true);

    let mut event_pump = engine.sdl.event_pump().unwrap();
    while engine.running {
        app.update(engine, engine.frame_counter.dt().as_secs_f32());

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        app.draw(engine);

        engine.g2d.draw(&engine.window_ortho_projection());

        engine.window.gl_swap_window();

        poll_event_pump(engine, app, &mut event_pump);

        let _ = engine.frame_counter.update();
    }
}

fn poll_event_pump<T: Application>(engine: &mut Engine, app: &mut T, event_pump: &mut EventPump) {
    use sdl3::event::{Event, WindowEvent};

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                engine.running = false;
                break;
            }

            Event::KeyDown {
                keycode,
                scancode,
                keymod,
                repeat,
                ..
            } => {
                app.key_event(
                    engine,
                    keycode,
                    scancode,
                    keymod,
                    if repeat {
                        input::KeyAction::Repeat
                    } else {
                        input::KeyAction::Press
                    },
                );
            }

            Event::KeyUp {
                keycode,
                scancode,
                keymod,
                ..
            } => {
                app.key_event(engine, keycode, scancode, keymod, input::KeyAction::Release);
            }

            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => app.mouse_button_event(
                engine,
                x,
                y,
                input::MouseButton::from(mouse_btn),
                input::MouseAction::Press,
            ),

            Event::Window {
                win_event: WindowEvent::PixelSizeChanged(width, height),
                ..
            } => unsafe {
                gl::Viewport(0, 0, width as _, height as _);
            },

            _ => {}
        }
    }
}

#[cfg(debug_assertions)]
use crate::gl::types::{GLchar, GLenum, GLsizei, GLuint, GLvoid};

#[cfg(debug_assertions)]
extern "system" fn gl_debug_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut GLvoid,
) {
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "api",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "window system",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "shader compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "third party",
        gl::DEBUG_SOURCE_APPLICATION => "application",
        gl::DEBUG_SOURCE_OTHER => "other",
        _ => "unknown",
    };

    let type_str = match gltype {
        gl::DEBUG_TYPE_ERROR => "error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "deprecated behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "undefined behavior",
        gl::DEBUG_TYPE_PORTABILITY => "portability",
        gl::DEBUG_TYPE_PERFORMANCE => "performance",
        gl::DEBUG_TYPE_MARKER => "marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "push group",
        gl::DEBUG_TYPE_POP_GROUP => "pop group",
        gl::DEBUG_TYPE_OTHER => "other",
        _ => "unknown",
    };

    let message_str = unsafe {
        std::ffi::CStr::from_ptr(message)
            .to_str()
            .unwrap()
            .to_owned()
    };
    let log_message =
        format!("(source: {source_str}) (type: {type_str}) (id: {id}): {message_str}");

    match severity {
        gl::DEBUG_SEVERITY_HIGH => log::error!("GLDEBUG (severity: high) {}", log_message),
        gl::DEBUG_SEVERITY_MEDIUM => log::warn!("GLDEBUG (severity: medium) {}", log_message),
        gl::DEBUG_SEVERITY_LOW => log::debug!("GLDEBUG (severity: low) {}", log_message),
        gl::DEBUG_SEVERITY_NOTIFICATION => log::trace!("GLDEBUG (severity: notif) {}", log_message),
        _ => unreachable!(),
    }
}
