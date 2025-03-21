use baphomet::{glm, hlgl};
use gl::types::GLsizei;
use glfw::{Action, Context, Key};
use rand::{Rng, rng};

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

    let mut vbo = hlgl::FVecBuffer::with_capacity(9 * 3);

    let vao = hlgl::VertexArrayBuilder::default()
        .attrib_pointer(&mut shader, &vbo, "aPos:3f aColor:3f aRot:3f")
        .build();

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let mut mvp = glm::ortho(0.0, 800.0, 600.0, 0.0, -1.0, 1.0);

    while !window.should_close() {
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

                    let cx = mx as f32;
                    let cy = my as f32;

                    let r = 25.0;

                    let x1 = cx + r * 270.0_f32.to_radians().cos();
                    let y1 = cy + r * 270.0_f32.to_radians().sin();

                    let x2 = cx + r * (270.0_f32 + 120.0).to_radians().cos();
                    let y2 = cy + r * (270.0_f32 + 120.0).to_radians().sin();

                    let x3 = cx + r * (270.0_f32 + 240.0).to_radians().cos();
                    let y3 = cy + r * (270.0_f32 + 240.0).to_radians().sin();

                    let dist = rand::distr::Uniform::new(0.0, 360.0).unwrap();
                    let theta: f32 = rng().sample(dist);

                    #[rustfmt::skip]
                    vbo.add([
                        x1, y1, 0.0,  1.0, 0.0, 0.0,  cx, cy, theta.to_radians(),
                        x2, y2, 0.0,  0.0, 1.0, 0.0,  cx, cy, theta.to_radians(),
                        x3, y3, 0.0,  0.0, 0.0, 1.0,  cx, cy, theta.to_radians(),
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
