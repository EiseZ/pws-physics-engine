use glium::Display;
use glium::IndexBuffer;

use glium::VertexBuffer;

use crate::vertex::Vertex;

pub struct Quad {
    pub vertex_buffer: VertexBuffer<Vertex>,
}

impl Quad {
    pub fn new(display: &Display, vertices: &[Vertex; 4]) -> Quad {
        Quad {
            vertex_buffer: glium::VertexBuffer::new(display, vertices).unwrap(),
        }
    }
}