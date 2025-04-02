pub use sdl3::keyboard::{Keycode, Mod, Scancode};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Unknown,
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseAction {
    Press,
    Release,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyAction {
    Press,
    Release,
    Repeat,
}

impl From<sdl3::mouse::MouseButton> for MouseButton {
    fn from(value: sdl3::mouse::MouseButton) -> Self {
        match value {
            sdl3::mouse::MouseButton::Left => Self::Left,
            sdl3::mouse::MouseButton::Middle => Self::Middle,
            sdl3::mouse::MouseButton::Right => Self::Right,
            _ => Self::Unknown,
        }
    }
}
