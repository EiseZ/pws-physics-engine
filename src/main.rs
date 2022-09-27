extern crate glium;
use glium::glutin;
use std::time::Instant;

mod init;
mod vertex;
mod renderer;
mod shader;
mod quad;
mod camera;
mod engine;
mod phycics_objects;
mod vector;
mod matrix;

use engine::Engine;
use phycics_objects::{PhysicsObject, PhysicsObjectType};
use vector::Vector;

const WIDTH: u32 = 800;
const HEIGTH: u32 = 600;

fn main() {
    let (event_loop, display) = init::init(WIDTH, HEIGTH);

    let mut engine = Engine::new(&display, WIDTH, HEIGTH, 0.01, 100.0);

    let mut rect1 = PhysicsObject::new(PhysicsObjectType::Rect, Vector::new(0.0, 0.0, 0.0), Vector::new(100.0, 100.0, 1.0), Vector::new(0.0, 0.0, 0.0));
    let mut rect2 = PhysicsObject::new(PhysicsObjectType::Rect, Vector::new(0.0, 0.0, 0.0), Vector::new(100.0, 100.0, 100.0), Vector::new(0.0, 0.0, 0.0));

    engine.objects.push(rect1);
    // engine.objects.push(rect2);

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

        engine.objects[0].rotate(Vector { x: 0.01, y: 0.0, z: 0.0 });
        engine.render_all(&display);

        time_last_rendered = Instant::now();
    });
}
