use crate::phycics_objects::{PhysicsObject, PhysicsObjectType};
use crate::camera::Camera;
use crate::shader;
use crate::vertex::Vertex;
use crate::quad::Quad;
use glium::{Display, Program};

pub struct Engine {
    // All phycics objects simulated
    pub objects: Vec<PhysicsObject>,
    camera: Camera,
    program: Program,
    quad: Quad,
}

impl Engine {
    pub fn new(display: &Display, width: u32, heigth: u32, near: f32, far: f32) -> Engine {
        return Engine {
            objects: Vec::new(),
            camera: Camera::new(display, width, heigth, near, far),
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
                    let matrix = [
                        [object.scale_x, 0.0, 0.0, 0.0],
                        [0.0, object.scale_y, 0.0, 0.0],
                        [0.0, 0.0, object.scale_z, 0.0],
                        [object.x, object.y, 0.0, 1.0],
                    ];
                    self.camera.render_rect(&mut target, &self.program, &self.quad, matrix)   
                },

                _ => { panic!("Circle") }
            }
        }
        target.finish().unwrap();
    }
}