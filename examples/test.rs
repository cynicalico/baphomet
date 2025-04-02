use baphomet::input::*;
use baphomet::{Application, Engine, Rgba, Ticker};
use rand::prelude::*;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = baphomet::init("TestApp", 800, 600, |builder| {
        builder.resizable().position_centered()
    })?;

    let mut app = TestApp {
        title_update: Ticker::new(Duration::from_millis(100)),
    };

    baphomet::run_app(&mut engine, &mut app);

    Ok(())
}

struct TestApp {
    title_update: Ticker,
}

impl Application for TestApp {
    fn update(&mut self, engine: &mut Engine, _dt: f32) {
        if self.title_update.tick() > 0 {
            let vsync_label = if engine.vsync() { " (vsync)" } else { "" };
            let _ = engine.window.set_title(&format!(
                "TestApp | {:.2} fps{}",
                engine.frame_counter.fps(),
                vsync_label
            ));
        }
    }

    fn draw(&mut self, _engine: &mut Engine) {}

    fn key_event(
        &mut self,
        engine: &mut Engine,
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

    fn mouse_button_event(
        &mut self,
        _engine: &mut Engine,
        x: f32,
        y: f32,
        button: MouseButton,
        action: MouseAction,
    ) {
        if button == MouseButton::Left && action == MouseAction::Press {
            let r = 25.0;
            let dist = rand::distr::Uniform::new(0.0, 360.0).unwrap();

            let x0 = x;
            let y0 = y + -r;
            let x1 = x + r * 0.866_025_4;
            let y1 = y + r * 0.5;
            let x2 = x + r * -0.866_025_4;
            let y2 = y + r * 0.5;
            let color = Rgba::hex(0xff0000ff);
            let theta: f32 = rand::rng().sample(dist);

            _engine
                .batcher
                .fill_tri(x0, y0, x1, y1, x2, y2, &color, x, y, theta);
        }
    }
}
