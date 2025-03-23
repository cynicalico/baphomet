use baphomet::input::{KeyAction, MouseAction, MouseButton};
use baphomet::{EMA, hlgl};
use gl::types::GLsizei;
use rand::prelude::*;
use sdl3::keyboard::{Keycode, Mod, Scancode};
use std::error::Error;
use std::time::Duration;

struct TestApp {
    shader: hlgl::Shader,
    vbo: hlgl::FVecBuffer,
    vao: hlgl::VertexArray,
    title_update: baphomet::Ticker,
}

impl baphomet::Application for TestApp {
    fn update(&mut self, engine: &mut baphomet::Engine, _dt: f32) {
        if self.title_update.tick() > 0 {
            engine
                .window
                .set_title(&format!("TestApp | {:.2} fps", engine.frame_counter.fps()));
        }
    }

    fn draw(&mut self, engine: &mut baphomet::Engine) {
        self.shader.use_program();
        self.shader
            .uniform_mat("proj", false, &engine.window_ortho_projection());

        unsafe {
            self.vbo.sync();

            self.vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, (self.vbo.size() / 9) as GLsizei);
            self.vao.unbind();
        }
    }

    fn key_event(
        &mut self,
        engine: &mut baphomet::Engine,
        keycode: Option<Keycode>,
        _scancode: Option<Scancode>,
        _mods: Mod,
        action: KeyAction,
    ) {
        match keycode {
            Some(Keycode::Escape) => engine.shutdown(),
            Some(Keycode::_1) => {
                if action == KeyAction::Press {
                    let is_vsync = engine.vsync();
                    engine.set_vsync(!is_vsync);
                }
            }
            _ => (),
        }
    }

    fn mouse_button_event(&mut self, x: f32, y: f32, button: MouseButton, action: MouseAction) {
        if button == MouseButton::Left && action == MouseAction::Press {
            let r = 25.0;

            let dist = rand::distr::Uniform::new(0.0, 360.0).unwrap();
            let theta: f32 = rand::rng().sample(dist);

            #[rustfmt::skip]
            self.vbo.add([
                x,                     y + -r,       0.0,  1.0, 0.0, 0.0,  x, y, theta.to_radians(),
                x + r * 0.866_025_4 ,  y + r * 0.5 , 0.0,  0.0, 1.0, 0.0,  x, y, theta.to_radians(),
                x + r * -0.866_025_4 , y + r * 0.5 , 0.0,  0.0, 0.0, 1.0,  x, y, theta.to_radians(),
            ]);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = baphomet::init("TestApp", 800, 600, |builder| {
        builder.resizable().position_centered()
    })?;

    let mut shader = hlgl::ShaderBuilder::default()
        .with_src_file(hlgl::ShaderKind::Vertex, "examples/res/basic.vert")?
        .with_src_file(hlgl::ShaderKind::Fragment, "examples/res/basic.frag")?
        .try_link()?;

    let vbo = hlgl::FVecBuffer::default();

    let vao = hlgl::VertexArrayBuilder::default()
        .attrib_pointer(&mut shader, &vbo, "aPos:3f aColor:3f aRot:3f")
        .build();

    baphomet::run_app(
        &mut engine,
        &mut TestApp {
            shader,
            vbo,
            vao,
            title_update: baphomet::Ticker::new(Duration::from_millis(100)),
        },
    );

    Ok(())
}
