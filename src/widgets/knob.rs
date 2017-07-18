use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;

pub struct Knob {
    position: Rect,
    color: Color,
}

impl Knob {
    pub fn new(rect: Rect) -> Knob {
        Knob {
            position: rect,
            color: color::RED,
        }
    }

    pub fn color(mut self, color: Color) -> Self { self.color = color; self }

    pub fn set(&self, renderer: &mut Renderer) {
        renderer.instructions.push(
            RenderElement::Circle(self.position.clone(), self.color)
        );
    }
}
