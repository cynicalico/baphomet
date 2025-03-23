use crate::{
    Engine,
    input::{KeyAction, MouseAction, MouseButton},
};

pub trait Application {
    fn update(&mut self, engine: &mut Engine, dt: f32);
    fn draw(&mut self, engine: &mut Engine);

    fn key_event(
        &mut self,
        _engine: &mut Engine,
        _keycode: Option<sdl3::keyboard::Keycode>,
        _scancode: Option<sdl3::keyboard::Scancode>,
        _mods: sdl3::keyboard::Mod,
        _action: KeyAction,
    ) {
    }
    fn mouse_button_event(&mut self, _x: f32, _y: f32, _button: MouseButton, _action: MouseAction) {
    }
}
