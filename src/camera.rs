use glium::{Program, Frame, Display};

use crate::renderer::Renderer;
use crate::quad::Quad;

pub struct Camera {
    renderer: Renderer,
    perspective: [[f32; 4]; 4]
}

impl Camera {
    pub fn new(display: &Display, width: u32, heigth: u32, near: f32, far: f32) -> Camera {
        let left = -(width as f32);
        let right = width as f32;
        let bottom = -(heigth as f32);
        let top = heigth as f32;
        Camera {
            renderer: Renderer::new(display),
            perspective: [
                [ 2.0 / (left - right), 0.0,                   0.0,                - (left + right) / (right - left) ],
                [ 0.0,                  2.0 / (top - bottom),  0.0,                - (top + bottom) / (top - bottom) ],
                [ 0.0,                  0.0,                  -2.0 / (far - near), - (far + near) / (far - near) ],
                [ 0.0,                  0.0,                   0.0,                1.0 ],
            ],
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