pub extern crate nalgebra_glm as glm;

pub mod application;
mod averagers;
pub mod hlgl;
pub mod input;
mod time;

pub use application::*;
pub use averagers::*;
use gl::types::GLsizei;
use sdl3::EventPump;
pub use time::*;

pub struct Engine {
    pub window: sdl3::video::Window,
    pub frame_counter: FrameCounter,

    sdl: sdl3::Sdl,
    #[allow(dead_code)]
    ctx: sdl3::video::GLContext,
    running: bool,
}

impl Engine {
    pub fn window_ortho_projection(&self) -> glm::Mat4 {
        glm::ortho(
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

    let mut builder = video_subsystem.window(title, width, height);
    let window = window_build_fn(&mut builder)
        .high_pixel_density()
        .opengl()
        .build()?;
    log::debug!(
        "Opened window (size: {:?}, pixel size: {:?})",
        window.size(),
        window.size_in_pixels()
    );

    let ctx = window.gl_create_context()?;
    window.gl_make_current(&ctx)?;

    gl::load_with(|name| match video_subsystem.gl_get_proc_address(name) {
        None => std::ptr::null(),
        Some(addr) => addr as *const _,
    });

    log::debug!(
        "OpenGL v{}.{}",
        gl_attr.context_version().0,
        gl_attr.context_version().1
    );

    Ok(Engine {
        window,
        frame_counter: FrameCounter::default(),
        sdl: sdl_context,
        ctx,
        running: false,
    })
}

pub fn run_app<T: Application>(engine: &mut Engine, app: &mut T) {
    engine.running = true;

    let mut event_pump = engine.sdl.event_pump().unwrap();
    while engine.running {
        app.update(engine, engine.frame_counter.dt().as_secs_f32());

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        app.draw(engine);

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
                x,
                y,
                input::MouseButton::from(mouse_btn),
                input::MouseAction::Press,
            ),

            Event::Window { win_event, .. } => match win_event {
                // WindowEvent::Resized(width, height) => {
                //     app.window_resized(width as u32, height as u32);
                // }
                WindowEvent::PixelSizeChanged(width, height) => unsafe {
                    gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
                },
                _ => {}
            },

            _ => {}
        }
    }
}
