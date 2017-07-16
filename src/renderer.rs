//use std::io;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Read;

use cgmath;

use glium;
use glium::Surface;
use glium::uniforms::*;
use widgets::*;

use Rect;
//use Point;
//use { Matrix2, Matrix3, Matrix4 };

#[derive(Clone)]
pub enum RenderElement {
    Triangle(Rect)
}

pub struct Renderer {
    pub display: glium::Display,
    pub triangle_program: glium::Program,
    pub instructions: Vec<RenderElement>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Renderer{
    pub fn new(display: glium::Display) -> Renderer {
        let triangle_program = program_from_shader(&display,
                                                   include_str!("shaders/polygon.vert"),
                                                   include_str!("shaders/polygon.frag"));
        let points = display.get_window().unwrap().get_inner_size_points().unwrap();
        Renderer {
            display: display,
            instructions: Vec::new(),
            triangle_program: triangle_program,
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
        target.clear_color(0.1, 0.1, 0.1, 1.0);

        let (view_width, view_height) = self.get_inner_size_points();
        let projection: [[f32; 4]; 4] = cgmath::ortho(0.0, view_width, 0.0, view_height, -1.0, 1.0).into();

        for instruction in self.instructions.clone() {
            match instruction {
                RenderElement::Triangle(position) => {

                    let uniforms = uniform! {
                        ortho_projection: projection,
                        u_resolution: [view_width, view_height],
                        scale_matrix: [
                            [ (position.width / view_width), 0., 0., 0. ], // x
                            [ 0., (position.height / view_height), 0., 0. ], // y
                            [ 0., 0., 1., 0. ], // z
                            [ 0., 0., 0., 1.0f32 ],
                        ],
                        offset_matrix: [
                            [ 1., 0., 0., (view_width / 2.0) + position.origin.x ], // x
                            [ 0., 1., 0., (view_height / 2.0) + position.origin.y ], // y
                            [ 0., 0., 1., 1. ], // z
                            [ 0., 0., 0., 1.0f32 ],
                        ]
                    };

                    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                    let (x1, y1, x2, y2) = position.coords();
                    let shape = vec![
                        Vertex { position: [ x1, y1 ] },
                        Vertex { position: [ x2, y2 ] },
                        Vertex { position: [ x1, y2 ] },
                        Vertex { position: [ x1, y1 ] },
                        Vertex { position: [ x2, y2 ] },
                        Vertex { position: [ x2, y1 ] },
                    ];

                    let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();

                    target.draw(
                        &vertex_buffer,
                        &indices,
                        &self.triangle_program,
                        &uniforms,
                        &Default::default()).unwrap();

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

