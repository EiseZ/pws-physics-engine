use glium::{Display, Program};
use std::fs;

pub fn create_program(display: Display) -> Program {
    // Read shaders from file
    let vertex_shader: &str = &fs::read_to_string("shaders/vs.vert").unwrap()[..];
    let fragment_shader: &str = &fs::read_to_string("shaders/fs.frag").unwrap()[..];

    
    glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap()
}