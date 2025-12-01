use std::{
    error::Error,
    io::stdout,
    sync::{
        atomic::{AtomicBool, Ordering}, mpsc, Arc
    },
};

use ratatui::{crossterm::terminal::size, prelude::CrosstermBackend, Terminal};
use ratatui_impl::{channels::{StdReceiver, StdSender}, context::RatContext, layouts::RatatuiRect};
use ui_core::{components::{Button, Component, Cons, Nil, TextBox}, inputs::Vec2, layouts::{HSplit, LayoutGenerator, Rectangle}, runtime::run_ui_loop};

use ratatui_impl::input_handler::StdInputHandler;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a Crossterm backend from stdout
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);

    // Create a shared quit flag
    let quit = Arc::new(AtomicBool::new(false));

    // Initialize the RatContext (your context implementation)
    let mut context = RatContext::new(backend, quit.clone())?;

    let (input_tx, input_rx) = mpsc::channel();
    let (button_tx, button_rx) = mpsc::channel();

    let (width, height) = size()?; // gets terminal size (cols, rows)
    let region = RatatuiRect::new(
        Vec2::new(0, 0),
        Vec2::new(width as u16, height as u16),
    );
    let split = HSplit;
    let [left, right] =
        <HSplit<1, 1> as LayoutGenerator<u16, RatatuiRect, 2>>::generate(&split, region);

    let root = TextBox::new("Hello World!".to_string(), &left, None)
        .push(
            Button::new(
                StdSender(button_tx), 
                StdReceiver(input_rx), 
                &right,
                "This is a button!".to_string(), 
                None, 
                )
            )
        .push(StdInputHandler::new(StdSender(input_tx), quit));

    // Example usage: render loop or app logic would go here
    run_ui_loop(root, &mut context);

    // When `context` goes out of scope, Drop will clean up the terminal
    Ok(())
}
