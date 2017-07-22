use glium::Frame;
//use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::glutin::Display;

//use Rect;

mod knob;
pub use self::knob::Knob;

mod button;
pub use self::button::Button;

pub trait Widget {
    fn new(display: &Display) -> Self;
    fn draw(&self, target: &mut Frame);
//    fn position(&self) -> Rect;
}
