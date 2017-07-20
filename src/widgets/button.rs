use renderer::RenderElement;
use Renderer;
use Rect;
use color;
use color::Color;
use Ui;
use ui::{ MouseButton, WidgetState, FocusedWidget, FocusedWidgetState };

use glutin;

pub struct Button {
    position: Rect,
    color: Color,
    state: ButtonState,
}

enum ButtonState {
    Idle,
    Hovered,
    Pressed,
}

impl Button {
    pub fn new(rect: Rect) -> Button {
        Button {
            position: rect,
            color: color::RED,
            state: ButtonState::Idle,
        }
    }

    pub fn color(mut self, color: Color) -> Self { self.color = color; self }

    pub fn draw(&self, renderer: &mut Renderer) {
        let color = match self.state {
            ButtonState::Hovered => { color::BLUE },
            _ => { self.color }
        };

        renderer.instructions.push(
            RenderElement::Triangle(self.position.clone(), color)
        );
    }

    pub fn handle(mut self, events: &[glutin::Event], ui: &mut Ui, id: String) -> Self {
        let mouse_inside = ui.mouse_inside_rect(self.position.clone());
        let is_focus = ui.is_focused(id.clone());

        if mouse_inside && !ui.is_locked() {
            ui.set_hovered(id.clone());
            self.state = ButtonState::Hovered;
        };

        self
    }

    pub fn clicked(self) -> bool {
        false
    }
}
