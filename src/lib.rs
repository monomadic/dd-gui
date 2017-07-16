#[macro_use] pub extern crate glium;

pub extern crate winit;
pub extern crate glutin;

pub extern crate cgmath;
pub type Matrix4 = cgmath::Matrix4<f64>;

mod renderer;
pub use renderer::Renderer;

pub mod widgets;

#[derive(Clone, Debug)]
pub struct Rect {
    pub origin: Point,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
