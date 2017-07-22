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
//        let color = match self.state {
//            ButtonState::Hovered => { color::BLUE },
//            ButtonState::Pressed => { color::RED },
//            _ => { self.color }
//        };

        renderer.instructions.push(
            RenderElement::Triangle(self.position.clone(), self.color)
        );
    }

    pub fn handle_events(mut self, events: &[glutin::Event], ui: &mut Ui, id: String) -> Self {
        let mouse_inside = ui.mouse_inside_rect(self.position.clone());
        let is_focus = ui.is_focused(id.clone());

//        for event in events {
//            match event {
//                &glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
//                    if mouse_inside && is_focus {
//                        println!("clicked: {}", id);
//                    }
//                },
//                _ => ()
//            }
//        }

        match ui.update(id.clone(), self.position.clone()) {
            Some(state) => {
                match state {
                    FocusedWidgetState::Hovered => { self.color = color::BLUE; }
                    FocusedWidgetState::Pressed => { self.color = color::RED; }
                    FocusedWidgetState::Activated => { self.color = color::BLUE; println!("clicked: {}", id.clone()); }
                }
            },
            None => ()
        };

        self
    }

    pub fn clicked(self) -> bool {
        false
    }
}
