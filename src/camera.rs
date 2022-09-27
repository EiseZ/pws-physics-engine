use glium::{Program, Frame, Display};

use crate::matrix::{multiply_matrix, multiply_matrix_to_row_major};
use crate::renderer::Renderer;
use crate::quad::Quad;

pub struct Camera {
    renderer: Renderer,
    perspective: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(display: &Display, width: u32, heigth: u32, near: f32, far: f32, fov: f32) -> Camera {
        let left = width as f32;
        let right = -(width as f32);
        let bottom = heigth as f32;
        let top = -(heigth as f32);


        let f = 1.0 / (fov / 2.0).tan();
        Camera {
            renderer: Renderer::new(display),
            perspective: [
                [ 1.0 / right, 0.0, 0.0, 0.0 ],
                [ 0.0, 1.0 / top,  0.0, 0.0 ],
                [ 0.0, 0.0, -2.0 / (far - near), - (far + near) / (far - near) ],
                [ 0.0, 0.0, 0.0, 1.0 ],
            ],
            // perspective: [
            //     [ near / right, 0.0, 0.0, 0.0 ],
            //     [ 0.0, near / top, 0.0, 0.0 ],
            //     [ 0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near)],
            //     [ 0.0, 0.0, -1.0, 0.0 ],
            // ],
            // perspective: [
            //     [ f * (heigth as f32 / width as f32), 0.0, 0.0, 0.0 ],
            //     [ 0.0, f, 0.0, 0.0 ],
            //     [ 0.0, 0.0, (far + near) / (far - near), 1.0 ],
            //     [ 0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
            // ],
            // perspective: multiply_matrix(
            //      Ortho:
            //     &[
            //         [ 2.0 / (left - right), 0.0,                   0.0,                - (left + right) / (right - left) ],
            //         [ 0.0,                  2.0 / (top - bottom),  0.0,                - (top + bottom) / (top - bottom) ],
            //         [ 0.0,                  0.0,                  -2.0 / (far - near), - (far + near) / (far - near) ],
            //         [ 0.0,                  0.0,                   0.0,                1.0 ],
            //     ],
            //      Perspective:
            //     &[
            //         [ near, 0.0, 0.0, 0.0 ],
            //         [ 0.0, near, 0.0, 0.0 ],
            //         [ 0.0, 0.0, far + near, -far * near ],
            //         [ 0.0, 0.0, 1.0, 0.0 ],
            //     ]
            // ),
        }
    }

    pub fn setup_frame(&self, display: &Display, r: f32, g: f32, b: f32) -> Frame {
        self.renderer.setup_target(&display, r, g, b)
    }

    pub fn render_rect(&self, target: &mut Frame, program: &Program, quad: &Quad, matrix: [[f32; 4]; 4]) {
        self.renderer.draw_quad(target, program, quad, matrix, &self.perspective);
    }

    pub fn finish(&self, target: Frame) {
        self.renderer.finish(target);
    }
}