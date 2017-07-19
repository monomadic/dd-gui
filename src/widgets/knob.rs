use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;
use Ui;
use ui::{ MouseButton, WidgetState };

use glutin;

pub struct Knob {
    position:   Rect,
    color:      Color,
    state:      WidgetState,
}

impl Knob {
    pub fn new(rect: Rect) -> Knob {
        Knob {
            position: rect,
            color: color::RED,
            state: WidgetState{ hovered: false, hot: false, activated: false },
        }
    }

    pub fn color(mut self, color: Color) -> Self { self.color = color; self }

    pub fn draw(mut self, renderer: &mut Renderer) -> Self {
        renderer.instructions.push(
            RenderElement::Circle(self.position.clone(), self.color)
        );
        self
    }

    pub fn handle(mut self, events: &[glutin::Event], ui: &mut Ui) -> Self {
        for event in events {
            match event {
                &glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                    self.state.hot = true;
                    if ui.mouse_inside_rect(self.position.clone()) {
                        println!("click!");
                    }
                }
                &glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
                    self.state.activated = true;
                }
                _ => ()
            }
        }

        self
    }

    pub fn mouse_up(self) -> bool {
        self.state.activated
    }
}
