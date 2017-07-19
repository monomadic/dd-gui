use glutin;
use Point;
use Rect;
use Renderer;

/// Ui:
/// Tracks the ui's global state across frames. As widgets do not track state across frames we
/// need a way of keeping track of multi-frame info eg. drags, mouse position.
#[derive(Clone, Debug)]
pub struct Ui {
    pub mouse:  MouseState,
    size:       (f32, f32),
}

/// WidgetState:
/// Tracks the current state of the widget on a per-frame basis.
#[derive(Clone, Copy, Debug)]
pub struct WidgetState {
    pub hovered: bool,      // about to interact, eg. mouse over, tab-order in
    pub hot: bool,          // engaged eg. mouse is down, possibly dragging
    pub activated: bool,    // actually interacted eg. mouse up while hovered, key down, etc
}

#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub position: Point,
    pub state: MouseButton,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MouseButton {
    Idle,
    Down,
    Up,
}

impl Ui {
    pub fn new(renderer: &mut Renderer) -> Self {
        let size = renderer.get_inner_size_points();
        Self {
            mouse: MouseState { position: Point{ x:0., y:0. }, state: MouseButton::Idle },
            size: size,
        }
    }

//    pub fn handle_glutin_events(mut self, renderer: Renderer) -> Self {
//        let events: Vec<glutin::Event> = renderer.display.poll_events().collect();
//        self
//    }

    pub fn handle_events(&mut self, events: &[glutin::Event]) {
        for event in events {
            if self.mouse.state == MouseButton::Up {
                self.mouse.state = MouseButton::Idle;
            }

            match event {
                &glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                    self.mouse.state = MouseButton::Down;
                }

                &glutin::Event::MouseInput(glutin::ElementState::Released, glutin::MouseButton::Left) => {
                    self.mouse.state = MouseButton::Up;
                }

                &glutin::Event::MouseMoved(x, y) => {
                    self.mouse.position = Point { x: x as f32, y: y as f32 };
                }

                _ => {
                    self.mouse.state = MouseButton::Idle;
                }
            }
        }
    }

    pub fn mouse_inside_rect(&mut self, rect: Rect) -> bool {
        let (x, y) = (
            (self.mouse.position.x / 2.),
            (self.size.1 - (self.mouse.position.y / 2.)));
        let (min, max) = rect.coords();

//        println!("testing: {:?}", (x, y, min, max));

        (x > min.x && y > min.y &&
            x < max.x && y < max.y)
    }
}
