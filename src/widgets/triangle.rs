use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;

pub struct Triangle {
    position: Rect,
    color: Color,
}

impl Triangle {
    pub fn new(rect: Rect) -> Triangle {
        Triangle {
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

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
