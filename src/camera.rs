use glium::{Display, Frame, Program};

use crate::matrix::{multiply_mat_vec, multiply_matrix, multiply_matrix_to_row_major};
use crate::quad::Quad;
use crate::renderer::Renderer;
use crate::Vector;

pub struct Camera {
    renderer: Renderer,
    perspective: [[f32; 4]; 4],
}

impl Camera {
    pub fn new(
        display: &Display,
        width: u32,
        height: u32,
        znear: f32,
        zfar: f32,
        fov: f32,
    ) -> Camera {
        // let left = -(width as f32);
        // let right = width as f32;
        // let bottom = -(height as f32);
        // let top = height as f32;

        let aspect_ratio = width as f32 / height as f32;

        let f = 1.0 / (fov / 2.0).tan();
        Camera {
            renderer: Renderer::new(display),
            perspective: [
                [f / aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, zfar / (zfar - znear), 1.0],
                [0.0, 0.0, (-zfar * znear) / (zfar - znear), 0.0],
            ],
            // Orhto
            // perspective: [
            //     [ 2.0 / (left - right), 0.0,                   0.0,                - (left + right) / (right - left) ],
            //     [ 0.0,                  2.0 / (top - bottom),  0.0,                - (top + bottom) / (top - bottom) ],
            //     [ 0.0,                  0.0,                  -2.0 / (far - near), - (far + near) / (far - near) ],
            //     [ 0.0,                  0.0,                   0.0,                1.0 ],
            // ],
        }
    }

    pub fn setup_frame(&self, display: &Display, r: f32, g: f32, b: f32) -> Frame {
        self.renderer.setup_target(&display, r, g, b)
    }

    pub fn render_rect(
        &self,
        target: &mut Frame,
        program: &Program,
        quad: &Quad,
        matrix: [[f32; 4]; 4],
    ) {
        self.renderer
            .draw_quad(target, program, quad, matrix, &self.perspective);
    }

    pub fn finish(&self, target: Frame) {
        self.renderer.finish(target);
    }
}
