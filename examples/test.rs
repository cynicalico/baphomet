use baphomet::{MouseAction, MouseButton, glm, hlgl};
use gl::types::GLsizei;
use rand::{Rng, rng};

struct TestApp {
    shader: hlgl::Shader,
    vbo: hlgl::FVecBuffer,
    vao: hlgl::VertexArray,
    proj: glm::Mat4,
}

impl baphomet::Application for TestApp {
    fn update(&mut self, dt: f32) {}

    fn draw(&mut self) {
        self.shader.use_program();
        self.shader.uniform_mat("mvp", false, &self.proj);

        unsafe {
            self.vbo.sync();

            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, (self.vbo.size() / 9) as GLsizei);
            self.vao.unbind();
        }
    }

    fn mouse_button_event(&mut self, x: f32, y: f32, button: MouseButton, action: MouseAction) {
        if button == MouseButton::Left && action == MouseAction::Press {
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
            self.vbo.add([
                x1, y1, 0.0,  1.0, 0.0, 0.0,  x, y, theta.to_radians(),
                x2, y2, 0.0,  0.0, 1.0, 0.0,  x, y, theta.to_radians(),
                x3, y3, 0.0,  0.0, 0.0, 1.0,  x, y, theta.to_radians(),
            ]);
        }
    }

    fn window_resized(&mut self, width: u32, height: u32) {
        self.proj = glm::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
    }

    fn window_pixel_resized(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
        }
    }
}

fn main() {
    let mut engine = baphomet::init();

    let mut shader = hlgl::ShaderBuilder::default()
        .with_src_file(hlgl::ShaderKind::Vertex, "examples/res/basic.vert")
        .expect("Failed to read vertex shader")
        .with_src_file(hlgl::ShaderKind::Fragment, "examples/res/basic.frag")
        .expect("Failed to read fragment shader")
        .try_link()
        .expect("Failed to build shader.");

    let vbo = hlgl::FVecBuffer::default();

    let vao = hlgl::VertexArrayBuilder::default()
        .attrib_pointer(&mut shader, &vbo, "aPos:3f aColor:3f aRot:3f")
        .build();

    let mut app = TestApp {
        shader,
        vbo,
        vao,
        proj: glm::identity(),
    };

    baphomet::run_app(&mut engine, &mut app);
}
