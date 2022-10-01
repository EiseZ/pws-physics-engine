extern crate glium;
use glium::glutin;
use std::time::Instant;

mod camera;
mod constants;
mod engine;
mod init;
mod matrix;
mod phycics_objects;
mod quad;
mod renderer;
mod shader;
mod vector;
mod vertex;

use engine::Engine;
use phycics_objects::{PhysicsObject, PhysicsObjectType};
use vector::Vector;

const WIDTH: u32 = 800;
const HEIGTH: u32 = 600;

fn main() {
    let (event_loop, display) = init::init(WIDTH, HEIGTH);

    let mut engine = Engine::new(&display, WIDTH, HEIGTH, 0.1, 100.0, 3.14159 / 2.0);

    let mut rect1 = PhysicsObject::new(
        PhysicsObjectType::Rect,
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(10.0, 10.0, 10.0),
        Vector::new(0.0, 0.0, 0.0),
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    engine.objects.push(rect1);

    const TIMESTEP: f32 = 0.001;

    let mut time_last_rendered = Instant::now();
    // Create the event loop
    // ? move || is gwn een function maken zonder naam, net zoals () => in javascript
    event_loop.run(move |event, _, control_flow| {
        // Do action based on event (eg. key input)
        // ? Match is soort van switch statement
        match event {
            glutin::event::Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {}
            },

            _ => {}
        }

        let frame_time = Instant::now() - time_last_rendered;
        if frame_time.as_secs_f32() < TIMESTEP {
            return;
        }

        for object in &mut engine.objects {
            // object.set_acceleration(Vector {
            //     x: 0.0,
            //     y: -constants::g,
            //     z: 0.0,
            // });
            object.rotate(Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });
            object.update_all(TIMESTEP);
        }

        engine.render_all(&display);

        time_last_rendered = Instant::now();
    });
}
