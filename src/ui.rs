use glutin;
use Point;
use Rect;
use Renderer;

/// Ui:
/// Tracks the ui's global state across frames. As widgets do not keep state across frames we
/// need a way of keeping track of multi-frame info eg. the mouses state, dragging, etc.
#[derive(Clone, Debug)]
pub struct Ui {
    pub mouse:              MouseState,
    size:                   (f32, f32),
    pub focused_widget:     Option<FocusedWidget>,
}

// todo: add time ago for state change. for tweens and animations.
#[derive(Clone, Debug)]
pub struct FocusedWidget {
    id: String,
    state: FocusedWidgetState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FocusedWidgetState {
    Hovered,
    Pressed,
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
    Down,
    Up,
}

impl Ui {
    pub fn new(renderer: &mut Renderer) -> Self {
        let size = renderer.get_inner_size_points();
        Self {
            mouse: MouseState { position: Point{ x:0., y:0. }, state: MouseButton::Up },
            size: size,
            focused_widget: None,
        }
    }

    pub fn set_hovered(&mut self, id: String) {
        self.focused_widget = Some(
            FocusedWidget {
                id: id,
                state: FocusedWidgetState::Hovered,
            }
        );
    }

    pub fn set_pressed(&mut self, id: String) {
        self.focused_widget = Some(
            FocusedWidget {
                id: id,
                state: FocusedWidgetState::Pressed,
            }
        );
    }

    pub fn clear_focus(&mut self) {
        self.focused_widget = None;
    }

    pub fn is_focused(&self, id: String) -> bool {
        match self.focused_widget {
            Some(ref w) => { w.id == id }
            None => { false }
        }
    }

    // a widget has control and has locked the focus. can be forcibly broken of course.
    pub fn is_locked(&self) -> bool {
        match self.focused_widget {
            Some(ref w) => { w.state == FocusedWidgetState::Pressed }
            None => { false }
        }
    }

    pub fn handle_events(&mut self, events: &[glutin::Event]) {
        for event in events {
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
                    self.mouse.state = MouseButton::Up;
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
