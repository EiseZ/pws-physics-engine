extern crate glium;
use glium::{glutin, Surface};

fn main() {
    // Create event loop for events (input, window movement, etc.)
    let mut events_loop = glutin::event_loop::EventLoop::new();

    // Window parameters
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_title("PWS Physics")
        .with_resizable(false);

    // Create opengl context
    let cb = glutin::ContextBuilder::new();

    // Create the display with given parameters and specify event loop
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    // Create the event loop
    // ? move || is gwn een function maken zonder naam, net zoals () => in javascript
    events_loop.run(move |event, _, control_flow| {
        // Create new secondary frame buffer
        let mut target = display.draw();

        target.clear_color(0.5, 0.0, 0.8, 1.0);

        // Switch secondary and primary buffers
        // ? .unwrap() betekend panic on error, als er potentieel een error is en je doet niet .unwrap() (of soortgelijke functie) dan compiled het niet
        target.finish().unwrap();


        // Set 60 fps
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(1000000000 / 60);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

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
    });

    println!("Hello, world!");
}
