#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn rgba(r:u8, g:u8, b:u8, a: u8) -> Color {
    Color { r:r, g:g, b:b, a:a }
}

pub fn rgb(r:u8, g:u8, b:u8) -> Color {
    Color { r:r, g:g, b:b, a: 255}
}

pub const RED: Color = Color { r:240, g:0, b:0, a:255 };
pub const GREEN: Color = Color { r:0, g:255, b:0, a:255 };

impl Color {
    pub fn as_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        )
    }
}
