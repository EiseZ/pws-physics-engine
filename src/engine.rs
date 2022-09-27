use crate::phycics_objects::{PhysicsObject, PhysicsObjectType};
use crate::camera::Camera;
use crate::shader;
use crate::vertex::Vertex;
use crate::quad::Quad;
use crate::matrix::multiply_matrix;

use glium::{Display, Program};

pub struct Engine {
    // All phycics objects simulated
    pub objects: Vec<PhysicsObject>,
    camera: Camera,
    program: Program,
    quad: Quad,
}

impl Engine {
    pub fn new(display: &Display, width: u32, heigth: u32, near: f32, far: f32, fov: f32) -> Engine {
        return Engine {
            objects: Vec::new(),
            camera: Camera::new(display, width, heigth, near, far, fov),
            program: shader::create_program(display.clone()),
            quad: Quad::new(&display, &[
                Vertex { position: [-0.5, -0.5] },
                Vertex { position: [-0.5,  0.5] },
                Vertex { position: [ 0.5, -0.5] },
                Vertex { position: [ 0.5,  0.5] },
            ]),
        };
    }

    pub fn render_all(&self, display: &Display) {
        let mut target = self.camera.setup_frame(display, 0.0, 0.0, 0.0);
        for object in &self.objects {
            match object.object_type {
                PhysicsObjectType::Rect => {
                    let a = object.rot.x;
                    let b = object.rot.y;
                    let c = object.rot.z;

                    // Row major
                    let rot_x_mat = [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, a.cos(), -a.sin(), 0.0],
                        [0.0, a.sin(), a.cos(), 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];
                    let rot_y_mat = [
                        [b.cos(), 0.0, b.sin(), 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [-b.sin(), 0.0, b.cos(), 1.0],
                    ];
                    let rot_z_mat = [
                        [c.cos(), -c.sin(), 0.0, 0.0],
                        [c.sin(), c.cos(), 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];

                    let trans_mat = [
                        [object.scale.x, 0.0, 0.0, object.pos.x],
                        [0.0, object.scale.y, 0.0, object.pos.y],
                        [0.0, 0.0, object.scale.z, object.pos.z],
                        [0.0, 0.0, 0.0, 1.0],
                    ];

                    let matrix = multiply_matrix(&multiply_matrix(&multiply_matrix(&rot_x_mat, &rot_y_mat), &rot_z_mat), &trans_mat);
                    self.camera.render_rect(&mut target, &self.program, &self.quad, matrix)   
                },

                _ => { panic!("Circle") }
            }
        }

        self.camera.finish(target);
    }
}