use glutin;
use Point;
use Rect;

#[derive(Clone, Copy)]
pub struct Ui {
    pub mouse: MouseState,
    size: (f32, f32),
}

#[derive(Clone, Copy)]
pub enum WidgetState {
    Idle,
    Hot,
    Active,
}

#[derive(Clone, Copy)]
pub struct MouseState {
    pub position: Point,
    pub state: MouseButton,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MouseButton {
    Idle,
    Down,
    Up,
}

impl Ui {
    pub fn new(size: (f32, f32)) -> Self { Self {
        mouse: MouseState { position: Point{ x:0., y:0. }, state: MouseButton::Idle },
        size: size,
    }}

    pub fn handle_glutin_event(&mut self, event: glutin::Event) {
        match event {
            glutin::Event::MouseInput(glutin::ElementState::Pressed, glutin::MouseButton::Left) => {
                self.mouse.state = MouseButton::Down; }

            glutin::Event::MouseMoved(x, y) => {
                self.mouse.position = Point{ x:x as f32, y:y as f32 }; }

            _ => ()
        }
    }

    pub fn mouse_inside_rect(self, rect: Rect) -> bool {
//        println!("{:?} {:?}", self.mouse.position, rect);
        let (x, y) = (
            (self.mouse.position.x / 2.),
            (self.size.1 - (self.mouse.position.y / 2.))
        );
        println!("{:?}", (x, y, rect.clone()));
        let (min, max) = rect.coords();

        if  x > min.x && y > min.y &&
            x < max.x && y < max.y {
            println!("hit");
            return true
        } else {
            return false
        }
    }
}
