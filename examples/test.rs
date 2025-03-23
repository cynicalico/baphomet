use baphomet::{glm, hlgl};
use gl::types::GLsizei;
use rand::{Rng, rng};
use sdl3::event::{Event, WindowEvent};
use sdl3::keyboard::Keycode;

fn main() {
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

    let mut shader = hlgl::ShaderBuilder::default()
        .with_src_file(hlgl::ShaderKind::Vertex, "examples/res/basic.vert")
        .expect("Failed to read vertex shader")
        .with_src_file(hlgl::ShaderKind::Fragment, "examples/res/basic.frag")
        .expect("Failed to read fragment shader")
        .try_link()
        .expect("Failed to build shader.");

    let mut vbo = hlgl::FVecBuffer::with_capacity(9 * 3);

    let vao = hlgl::VertexArrayBuilder::default()
        .attrib_pointer(&mut shader, &vbo, "aPos:3f aColor:3f aRot:3f")
        .build();

    unsafe {
        gl::Viewport(
            0,
            0,
            window.size_in_pixels().0 as GLsizei,
            window.size_in_pixels().1 as GLsizei,
        );
    }

    let mut mvp = glm::ortho(
        0.0,
        window.size().0 as f32,
        window.size().1 as f32,
        0.0,
        -1.0,
        1.0,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader.use_program();
        shader.uniform_mat("mvp", false, &mvp);

        unsafe {
            vbo.sync();

            vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, (vbo.size() / 9) as GLsizei);
            vao.unbind();
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown { x, y, .. } => {
                    let r = 25.0;

                    let x1 = x + r * 270.0_f32.to_radians().cos();
                    let y1 = y + r * 270.0_f32.to_radians().sin();

                    let x2 = x + r * (270.0_f32 + 120.0).to_radians().cos();
                    let y2 = y + r * (270.0_f32 + 120.0).to_radians().sin();

                    let x3 = x + r * (270.0_f32 + 240.0).to_radians().cos();
                    let y3 = y + r * (270.0_f32 + 240.0).to_radians().sin();

                    let dist = rand::distr::Uniform::new(0.0, 360.0).unwrap();
                    let theta: f32 = rng().sample(dist);

                    #[rustfmt::skip]
                    vbo.add([
                        x1, y1, 0.0,  1.0, 0.0, 0.0,  x, y, theta.to_radians(),
                        x2, y2, 0.0,  0.0, 1.0, 0.0,  x, y, theta.to_radians(),
                        x3, y3, 0.0,  0.0, 0.0, 1.0,  x, y, theta.to_radians(),
                    ]);
                }

                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::PixelSizeChanged(width, height) => unsafe {
                        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
                    },
                    WindowEvent::Resized(width, height) => {
                        mvp = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
