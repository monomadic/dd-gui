use renderer::RenderElement;

use Renderer;
use Rect;
use color;
use color::Color;
use Ui;
use ui::{ MouseButton, WidgetState, FocusedWidget, FocusedWidgetState };

use glutin;

pub struct Knob {
    position:   Rect,
    color:      Color,
    state:      KnobState,
}

pub enum KnobState {
    Idle,
    Hovered,
    Turning,
}

impl Knob {
    pub fn new(rect: Rect) -> Knob {
        Knob {
            position: rect,
            color: color::RED,
            state: KnobState::Idle,
        }
    }

    pub fn color(mut self, color: Color) -> Self { self.color = color; self }

    pub fn draw(self, renderer: &mut Renderer) -> Self {
        renderer.instructions.push(
            RenderElement::Circle(self.position.clone(), self.color)
        );
        self
    }

    pub fn handle(mut self, events: &[glutin::Event], ui: &mut Ui, id: String) -> Self {
        let mouse_inside = ui.mouse_inside_rect(self.position.clone());
        let is_focus = ui.is_focused(id.clone());



        if !ui.is_locked() {
            if mouse_inside {
                ui.set_hovered(id.clone());
            }

        };

        for event in events {
            match event {
                &glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                    if mouse_inside {
                        ui.set_pressed(id.clone());
                        self.state = KnobState::Turning;
                        println!("knob widget is turning");
                    }
                }
                &glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
                    if is_focus {
                        ui.clear_focus();
                        self.state = KnobState::Idle; // we still need to care about this because we're processing a batch of events.
                        println!("knob widget stopped turning");
                    }
                }
                _ => ()
            }
        }

        self
    }

    pub fn changed(self) -> bool {
        false
    }
}
