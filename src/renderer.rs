use glium::IndexBuffer;
use glium::{index::NoIndices, uniform, Display, Frame, Program, Surface, VertexBuffer};

use crate::matrix::multiply_matrix_to_row_major;
use crate::quad::Quad;
use crate::vertex::Vertex;

pub struct Renderer {
    // Basic traigle indices
    triangle_indices: NoIndices,
    quad_indices: IndexBuffer<u32>,
}

impl Renderer {
    pub fn new(display: &Display) -> Renderer {
        Renderer {
            triangle_indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            quad_indices: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &[0, 1, 2, 2, 1, 3],
            )
            .unwrap(),
        }
    }

    // Run before every frame
    pub fn setup_target(&self, display: &Display, r: f32, g: f32, b: f32) -> Frame {
        // Create new secondary frame buffer
        let mut target = display.draw();

        // Set background color
        target.clear_color(r, g, b, 1.0);

        target
    }

    pub fn draw_triangle(
        &self,
        target: &mut Frame,
        program: &Program,
        vertex_buffer: &VertexBuffer<Vertex>,
        matrix: [[f32; 4]; 4],
    ) {
        // Create uniform
        let uniform = uniform! {
            matrix: matrix,
        };

        // Draw triangle
        target
            .draw(
                vertex_buffer,
                self.triangle_indices,
                program,
                &uniform,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn draw_quad(
        &self,
        target: &mut Frame,
        program: &Program,
        quad: &Quad,
        matrix: [[f32; 4]; 4],
        perspective: &[[f32; 4]; 4],
    ) {
        // Create uniform
        let uniform = uniform! {
            matrix: matrix,
            perspective: *perspective,
        };

        // Draw quad
        target
            .draw(
                &quad.vertex_buffer,
                &self.quad_indices,
                program,
                &uniform,
                &Default::default(),
            )
            .unwrap();
    }

    pub fn finish(&self, target: Frame) {
        // Switch secondary and primary buffers
        target.finish().unwrap();
    }
}
