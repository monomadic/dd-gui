use cgmath;
use glium;
use glium::Surface;

use Rect;
use color::Color;

#[derive(Clone)]
pub enum RenderElement {
    Triangle(Rect, Color),
    Circle(Rect, Color),
}

pub struct Renderer {
    pub display: glium::Display,
    pub triangle_program: glium::Program,
    pub circle_program: glium::Program,
    pub instructions: Vec<RenderElement>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Renderer {
    pub fn new(display: glium::Display) -> Renderer {
        let triangle_program = program_from_shader(&display,
                                                   include_str!("shaders/polygon.vert"),
                                                   include_str!("shaders/polygon.frag"));

        let circle_program = program_from_shader(&display,
                                                   include_str!("shaders/polygon.vert"),
                                                   include_str!("shaders/circle.frag"));
        Renderer {
            display: display,
            instructions: Vec::new(),
            triangle_program: triangle_program,
            circle_program: circle_program,
        }
    }

    pub fn get_inner_size_points(&mut self) -> (f32, f32) {
        let points = self.display.get_window()
            .expect("window to exist")
            .get_inner_size_points()
            .expect("window to exist");
        (points.0 as f32, points.1 as f32)
    }

    pub fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.005, 0.005, 0.005, 1.0);

        let (view_width, view_height) = self.get_inner_size_points();
        let projection: [[f32; 4]; 4] = cgmath::ortho(0.0, view_width, 0.0, view_height, -1.0, 1.0).into();

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        for instruction in self.instructions.clone() {
            match instruction {
                RenderElement::Circle(position, color) => {
                    let (min, max) = position.coords();

                    let uniforms = uniform! {
                        ortho_projection: projection,
                        u_resolution: [view_width, view_height],
                        u_color: color.as_f32(),
                        u_position: [(min.x + max.x), (min.y + max.y)],
                        u_radius: position.size.w() / 2.,
                    };

                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    target.draw(
                        &quad(&self.display, position),
                        &indices,
                        &self.circle_program,
                        &uniforms,
                        &draw_params).unwrap();
                },
                RenderElement::Triangle(position, color) => {

                    let uniforms = uniform! {
                        ortho_projection: projection,
                        u_resolution: [view_width, view_height],
                        u_color: color.as_f32(),
                    };

                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    target.draw(
                        &quad(&self.display, position),
                        &indices,
                        &self.triangle_program,
                        &uniforms,
                        &draw_params).unwrap();

                }
            }
        }
        target.finish().expect("target to unwrap");
        self.instructions = Vec::new(); // clear the instruction queue.
    }
}

pub fn program_from_shader(display: &glium::Display, vertex_shader: &str, fragment_shader: &str) -> glium::Program {
    glium::Program::from_source(
        display,
        &vertex_shader,
        &fragment_shader,
        None).expect("program to complete.")
}

fn quad(display: &glium::Display, position: Rect) -> glium::VertexBuffer<Vertex> {
    let (min, max) = position.coords();
    let shape = vec![
        Vertex { position: [min.x, min.y] },
        Vertex { position: [max.x, max.y] },
        Vertex { position: [min.x, max.y] },
        Vertex { position: [min.x, min.y] },
        Vertex { position: [max.x, max.y] },
        Vertex { position: [max.x, min.y] },
    ];
    glium::VertexBuffer::new(display, &shape).unwrap()
}
