use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;

mod knob;
pub use self::knob::Knob;

mod triangle;
pub use self::triangle::Triangle;

pub trait Widget {
    fn new(display: &GlutinFacade) -> Self;
    fn draw(&self, target: &mut Frame);
}
