use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;

pub struct Button {
    position: Rect,
    color: Color,
}

impl Button {
    pub fn new(rect: Rect) -> Button {
        Button {
            position: rect,
            color: color::RED,
        }
    }

    pub fn color(mut self, color: Color) -> Self { self.color = color; self }

    pub fn set(&self, renderer: &mut Renderer) {
        renderer.instructions.push(
            RenderElement::Triangle(self.position.clone(), self.color)
        );
    }
}
