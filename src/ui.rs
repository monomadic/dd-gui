use glutin;
use Point;
use Rect;
use Renderer;

/// Ui:
/// Tracks the ui's global state across frames. As widgets do not keep state across frames we
/// need a way of keeping track of multi-frame info eg. the mouses state, dragging, etc.
#[derive(Clone, Debug)]
pub struct Ui {
    size:                   (f32, f32),
    pub mouse:              MouseState,
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
    Activated,
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
            focused_widget: None,
        }
    }

    pub fn is_mouse_down(&self) -> bool {
        self.mouse.state == MouseButton::Down
    }

    pub fn set_hovered(&mut self, id: String) {
        self.focused_widget = Some(
            FocusedWidget {
                id: id,
                state: FocusedWidgetState::Hovered,
            }
        );
    }

    /// Updates widget state against ui
    pub fn update(&mut self, id: String, position: Rect) -> Option<FocusedWidgetState> {
        let mouse_inside = self.mouse_inside_rect(position);
        let is_focus = self.is_focused(id.clone());
        let mouse_is_down = self.is_mouse_down();
        let ui_locked = self.is_locked();
        let mouse_was_clicked = self.mouse.state == MouseButton::Up;

        if mouse_was_clicked && is_focus {
            // a mouseup occurred somewhere in the widget.
            self.clear_focus();
            if mouse_inside {
                // widget has completed a click.
                return Some(FocusedWidgetState::Activated);
            } else {
                // user cancelled by mousing up outside the widget.
                return None;
            }
        }

        if mouse_inside && !ui_locked {
            if mouse_is_down {
                // mouse has been depressed on the widget.
                self.set_pressed(id.clone());
                return Some(FocusedWidgetState::Pressed);
            } else {
                // mouse is hovering over widget.
                self.set_hovered(id.clone());
                return Some(FocusedWidgetState::Hovered);
            }
        };

        if is_focus && mouse_is_down {
            // mouse was depressed on the widget and has moved outside the widget.
            return Some(FocusedWidgetState::Pressed);
        }

        None
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

    // a widget has control and has locked the focus. can be forcibly broken.
    pub fn is_locked(&self) -> bool {
        match self.focused_widget {
            Some(ref w) => { w.state == FocusedWidgetState::Pressed }
            None => { false }
        }
    }

    pub fn handle_events(&mut self, events: &[glutin::Event]) {
        use glutin::Event::{ MouseInput, MouseMoved };
        use glutin::{ ElementState };

        // reset the mouse button on a mouseup from the previous frame, otherwise it will click forever.
        if self.mouse.state == MouseButton::Up {
            self.mouse.state = MouseButton::Idle;
        }

        // updates mouse state
        for event in events {
            match event {
                &MouseInput(ElementState::Pressed, glutin::MouseButton::Left) => {
//                    println!("{:?}", event);
                    self.clear_focus();
                    self.mouse.state = MouseButton::Down;
                }

                &MouseMoved(x, y) => {
                    self.mouse.position = Point { x: x as f32, y: y as f32 };
                }

                &MouseInput(ElementState::Released, glutin::MouseButton::Left) => {
                    self.mouse.state = MouseButton::Up;
                }

                _ => {
//                    println!("{:?}", event);
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
