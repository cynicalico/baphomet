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
        point_timer: Ticker::new(Duration::from_millis(10)),
    };

    baphomet::run_app(&mut engine, &mut app);

    Ok(())
}

struct TestApp {
    title_update: Ticker,
    point_timer: Ticker,
}

fn rand_color() -> Rgba {
    Rgba::new(
        rand::rng().random_range(..=255),
        rand::rng().random_range(..=255),
        rand::rng().random_range(..=255),
        255,
    )
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

    fn draw(&mut self, engine: &mut Engine) {
        for _ in 0..self.point_timer.tick() {
            engine.batcher.point(
                (
                    rand::rng().random_range(..engine.window.size().0) as f32,
                    rand::rng().random_range(..engine.window.size().1) as f32,
                ),
                &rand_color(),
            );
        }
    }

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
        engine: &mut Engine,
        x: f32,
        y: f32,
        button: MouseButton,
        action: MouseAction,
    ) {
        if button == MouseButton::Left && action == MouseAction::Press {
            let r = 25.0;
            engine.batcher.fill_tri(
                (x, y + -r),
                (x + r * 0.866_025_4, y + r * 0.5),
                (x + r * -0.866_025_4, y + r * 0.5),
                &rand_color(),
                (x, y),
                rand::rng().random_range(0.0..360.0),
            );
        } else if button == MouseButton::Right && action == MouseAction::Press {
            engine.batcher.line((0.0, 0.0), (x, y), &rand_color());
        }
    }
}
