extern crate glium;
use glium::glutin;
use quad::Quad;
use std::time::Instant;

mod init;
mod vertex;
mod renderer;
mod shader;
mod quad;

use vertex::Vertex;
use renderer::Renderer;

fn main() {
    let (event_loop, display) = init::init();

    let renderer = Renderer::new(&display);

    // Specify vertexes for triangle
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    // Put shape in vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Compile shaders into program
    let program = shader::create_program(display.clone());


    let quad = Quad::new(&display, &[
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [-1.0,  0.5] },
        Vertex { position: [ 0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
    ]);


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

        let mut target = renderer.setup_target(&display, 0.0, 0.0, 0.0);

        let matrix = [
            [ t.cos(),  t.sin(), 0.0, 0.0],
            [ -t.sin(), t.cos(), 0.0, 0.0],
            [ 0.0,      0.0,     1.0, 0.0],
            [ t,        0.0,     0.0, 1.0],
        ];
        //renderer.draw_triangle(&mut target, &program, &vertex_buffer, matrix);
        renderer.draw_quad(&mut target, &program, &quad, matrix);

        renderer.finish(target);

        time_last_rendered = Instant::now();
    });
}
