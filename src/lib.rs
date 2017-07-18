#[allow(unused_imports)]

#[macro_use] pub extern crate glium;

pub extern crate winit;
pub extern crate glutin;


pub extern crate cgmath;
pub type Matrix2 = cgmath::Matrix2<f64>;
pub type Matrix3 = cgmath::Matrix3<f64>;
pub type Matrix4 = cgmath::Matrix4<f64>;

mod renderer;
pub use renderer::Renderer;

pub mod widgets;
pub mod color;

#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn coords(&self) -> (f32, f32, f32, f32) {
        (
            self.origin.x,                  // x1
            self.origin.y,                  // y1
            self.origin.x + self.width,     // x2
            self.origin.y + self.height,    // y2
        )
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
