extern crate glium;
use glium::glutin;
use glium::Surface;
use glium::uniform;
use std::fs;
use std::time::Instant;

mod init;
mod vertex;

use vertex::Vertex;

fn main() {
    let (event_loop, display) = init::init();

    let vertex_shader: &str = &fs::read_to_string("shaders/vs.vert").unwrap()[..];
    let fragment_shader: &str = &fs::read_to_string("shaders/fs.frag").unwrap()[..];

    // Specify vertexes for triangle
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    // Put shape in vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Specify indicices (simple method because online triangle)
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Compile shaders into program
    let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    // T is uniform to change the position without changing vertexes
    let mut t: f32 = 0.0;

    let mut time_last_rendered = Instant::now();

    // Create the event loop
    // ? move || is gwn een function maken zonder naam, net zoals () => in javascript
    event_loop.run(move |event, _, control_flow| {
        // Do action based on event (eg. key input)
        // ? Match is soort van switch statement
        match event {
            glutin::event::Event::WindowEvent { event: window_event, .. } => {
                match window_event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => {},
                }
            },
            
            _ => {}
        }

        if Instant::now() - time_last_rendered < std::time::Duration::from_nanos(100000000 / 60) {
            return;
        }

        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        // Create new secondary frame buffer
        let mut target = display.draw();
        target.clear_color(0.5, 0.0, 0.8, 1.0);

        let uniform = uniform! {
            matrix: [
                [ t.cos(),  t.sin(), 0.0, 0.0],
                [ -t.sin(), t.cos(), 0.0, 0.0],
                [ 0.0,      0.0,     1.0, 0.0],
                [ t,        0.0,     0.0, 1.0],
            ]
        };

        // Draw triangle
        target.draw(&vertex_buffer, &indices, &program, &uniform, &Default::default()).unwrap();

        // Switch secondary and primary buffers
        // ? .unwrap() betekend panic on error, als er potentieel een error is en je doet niet .unwrap() (of soortgelijke functie) dan compiled het niet
        target.finish().unwrap();

        time_last_rendered = Instant::now();
    });

    println!("Hello, world!");
}
