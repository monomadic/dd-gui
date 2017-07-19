use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;
use Ui;
use ui::MouseButton;

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

    pub fn draw(mut self, renderer: &mut Renderer) -> Self {
        renderer.instructions.push(
            RenderElement::Circle(self.position.clone(), self.color)
        );
        self
    }

    pub fn handle(mut self, ui: &mut Ui) -> Self {
        self
    }

    pub fn clicked(self, ui: Ui) -> bool {
        ui.mouse_inside_rect(self.position) && ui.mouse.state == MouseButton::Down
    }
}
