use baphomet::{glm, hlgl, hlgl::GlBuffer};
use gl::types::{GLsizei, GLsizeiptr};
use glfw::{Action, Context, Key};
use rand::distr::Distribution;
use rand::{Rng, random, rng};
use std::error::Error;

fn main() {
    colog::init();

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    log::debug!("Initialized GLFW");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    log::debug!("Opened window");
    window.make_current();

    window.set_all_polling(true);

    gl::load_with(|s| window.get_proc_address(s).cast());

    let (gl_version_major, gl_version_minor) = unsafe {
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
        (major, minor)
    };
    log::debug!("OpenGL v{}.{}", gl_version_major, gl_version_minor);

    let mut shader = hlgl::ShaderBuilder::default()
        .with_src_file(hlgl::ShaderKind::Vertex, "examples/res/basic.vert")
        .expect("Failed to read vertex shader")
        .with_src_file(hlgl::ShaderKind::Fragment, "examples/res/basic.frag")
        .expect("Failed to read fragment shader")
        .try_link()
        .expect("Failed to build shader.");

    let mut vbo = hlgl::FVecBuffer::with_capacity(6);

    let vao = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        vbo.bind();

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * size_of::<f32>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * size_of::<f32>()) as GLsizei,
            (3 * size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        vao
    };

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let mut mvp = glm::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader.use_program();
        shader.uniform_mat("mvp", false, &mvp);

        unsafe {
            vbo.sync();

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, (vbo.size() / 6) as GLsizei);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    vbo.clear();
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButton::Left, Action::Press, _) => {
                    let (mx, my) = window.get_cursor_pos();
                    let mx = mx as f32;
                    let my = my as f32;
                    #[rustfmt::skip]
                    vbo.add([
                        mx,        my,        0.0, 1.0, 0.0, 0.0,
                        mx + 50.0, my,        0.0, 0.0, 1.0, 0.0,
                        mx + 50.0, my + 50.0, 0.0, 0.0, 0.0, 1.0,
                    ]);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                    mvp = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
                },
                _ => {}
            }
        }
    }
}
