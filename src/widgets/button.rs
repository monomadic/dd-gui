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

    pub fn handle(mut self, events: &[glutin::Event], ui: &mut Ui, id: String) -> Self {
        let mouse_inside = ui.mouse_inside_rect(self.position.clone());
        let is_focus = ui.is_focused(id.clone());
        let mouse_is_down = ui.is_mouse_down();

//        let mut button_is_down = false;
//        for event in events {
//            match event {
//                &glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
//                    if mouse_inside {
//                        button_is_down = true;
//                    }
//                },
//                _ => ()
//            }
//        }



        if mouse_inside {
            if !ui.is_locked() {
                if mouse_is_down {
                } else {
                    ui.set_hovered(id.clone());
                    self.color = color::BLUE;
                }
            }
        }
        if is_focus && mouse_is_down {
            ui.set_pressed(id.clone());
            self.color = color::RED;
        }

        self
    }

    pub fn clicked(self) -> bool {
        false
    }
}
