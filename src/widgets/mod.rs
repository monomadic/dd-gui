use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;

//use Rect;

mod knob;
pub use self::knob::Knob;

mod button;
pub use self::button::Button;

pub trait Widget {
    fn new(display: &GlutinFacade) -> Self;
    fn draw(&self, target: &mut Frame);
//    fn position(&self) -> Rect;
}
