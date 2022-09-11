use glium::{glutin, implement_vertex};

use crate::vertex::Vertex;

// Inits the required things to create a window and event loop
pub fn init() -> (glutin::event_loop::EventLoop<()>, glium::Display) {
    // Create event loop for events (input, window movement, etc.)
    let event_loop = glutin::event_loop::EventLoop::new();

    // Window parameters
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0))
        .with_title("PWS Physics")
        .with_resizable(false);

    // Create opengl context
    let cb = glutin::ContextBuilder::new();

    // Create the display with given parameters and specify event loop
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Specify vertex struct
    implement_vertex!(Vertex, position);

    (event_loop, display) 
}