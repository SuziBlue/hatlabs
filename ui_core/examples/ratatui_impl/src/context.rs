use ratatui::crossterm::cursor::{Hide, MoveTo, Show};
use ratatui::crossterm::{cursor, execute};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use ui_core::components::Component;
use ui_core::{CanRender, Context, RenderError};

use ratatui::{
    Terminal,
    backend::Backend,
};
use std::cell::RefCell;
use std::error::Error;
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::renderers::RatatuiFrame;

pub struct RatContext<B: Backend> {
    terminal: Terminal<B>,
    quit: Arc<AtomicBool>,
}

impl<B: Backend> RatContext<B> {
    pub fn new(backend: B, quit: Arc<AtomicBool>) -> Result<Self, Box<dyn Error>> {
        // Enable raw mode for terminal UI
        enable_raw_mode()?;

        // Hide cursor
        execute!(
            stdout(),
            EnterAlternateScreen,
            Hide,
            Clear(ClearType::All),
            MoveTo(0, 0),
        )?;

        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            quit,
        })
    }
}

impl<B: Backend> Drop for RatContext<B> {
    fn drop(&mut self) {
        // Best-effort terminal restoration
        let _ = disable_raw_mode();

        // Leave the alternate screen and restore cursor
        let _ = execute!(
            stdout(),
            LeaveAlternateScreen,
            Show,
            Clear(ClearType::All),
            MoveTo(0, 0),
        );

        let _ = self.terminal.show_cursor();
    }
}

impl<B, C> CanRender<C> for RatContext<B>
where
    B: Backend,
    C: Component,
    for<'outer, 'inner> RatatuiFrame<'outer, 'inner>: CanRender<C>,
{
    fn render(&mut self, component: &C) -> Result<(), RenderError> {
        let render_result: RefCell<Result<(), RenderError>> = RefCell::new(Ok(()));

        self.terminal.draw(|frame| {
            let mut rat_frame = RatatuiFrame { frame };
            *render_result.borrow_mut() = rat_frame.render(component);
        }).map_err(|_| RenderError)?;

        render_result.into_inner()
    }
}

impl<B: Backend> Context for RatContext<B> {
    fn should_quit(&self) -> bool {
        self.quit.load(Ordering::Relaxed)
    }

    fn render_setup(&self) -> Result<(), RenderError> {
        Ok(())
    }

    fn handle_error(&self, err: impl Error) {
        eprintln!("UI error: {:?}", err);
    }

    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}
