#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseAction {
    Press,
    Release,
}

pub trait Application {
    fn update(&mut self, dt: f32);
    fn draw(&mut self);

    fn key_event(&mut self) {}
    fn mouse_button_event(&mut self, x: f32, y: f32, button: MouseButton, action: MouseAction) {}

    fn window_resized(&mut self, width: u32, height: u32) {}
    fn window_pixel_resized(&mut self, width: u32, height: u32) {}
}
