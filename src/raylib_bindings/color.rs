#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

pub const BLACK: Color = Color::new(0, 0, 0, 255);
pub const DARKGRAY: Color = Color::new(80, 80, 80, 255);
pub const GREEN: Color = Color::new(0, 255, 0, 255);
pub const LIGHTGRAY: Color = Color::new(200, 200, 200, 255);
pub const RAYWHITE: Color = Color::new(245, 245, 245, 255);
pub const RED: Color = Color::new(255, 0, 0, 255);
pub const WHITE: Color = Color::new(255, 255, 255, 255);
