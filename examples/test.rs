use baphomet::{glm, hlgl};
use gl::types::{GLsizei, GLsizeiptr};
use glfw::{Action, Context, Key};

const VERT_SRC: &str = "\
#version 330
layout (location = 0) in vec3 aPos;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

const FRAG_SRC: &str = "\
#version 330 core
in vec3 aColor;

out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
";

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

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);

    window.set_framebuffer_size_callback(|_window, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

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
        .with_src(hlgl::ShaderKind::Vertex, VERT_SRC)
        .with_src(hlgl::ShaderKind::Fragment, FRAG_SRC)
        .try_link()
        .expect("Failed to link shader.");

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        50.0,  50.0,  0.0,
        100.0, 50.0,  0.0,
        100.0, 100.0, 0.0,
        50.0,  100.0, 0.0
    ];
    #[rustfmt::skip]
    let indices: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let vao = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        let mut ebo: u32 = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<f32>()) as GLsizeiptr,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        vao
    };

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader.use_program();
        shader.uniform_mat(
            "mvp",
            false,
            &glm::ortho(
                0.0,
                window.get_size().0 as f32,
                window.get_size().1 as f32,
                0.0,
                -1.0,
                1.0,
            ),
        );

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::MouseButton(glfw::MouseButtonLeft, Action::Press, _) => {
                    let (xpos, ypos) = window.get_cursor_pos();
                    log::info!("Got left button ({xpos:.1}, {ypos:.1})");
                }
                _ => {}
            }
        }
    }
}
