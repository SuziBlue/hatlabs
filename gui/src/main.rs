use std::error::Error;

use winit::dpi::{LogicalPosition, LogicalSize, Position};
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
struct WindowApp {
    event_loop: EventLoop<()>,
    window: Window,
}

impl WindowApp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("parent window")
            .with_position(Position::Logical(LogicalPosition::new(0.0, 0.0)))
            .with_inner_size(LogicalSize::new(640.0f32, 480.0f32))
            .build(&event_loop)?;
        event_loop.set_control_flow(ControlFlow::Wait);

        Ok(Self { event_loop, window })
    }

    fn run(self) -> Result<(), EventLoopError> {
        self.event_loop.run(|event, elwt| {
            WindowApp::event_handler(&self.window, event, elwt);
        })
    }
    fn event_handler(window: &Window, event: Event<()>, elwt: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window close requested");
                elwt.exit();
            }

            Event::AboutToWait => {
                window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                println!("Redrawing window")
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = WindowApp::new()?;
    app.run()?;
    Ok(())
}
