use core::{error::Error, fmt, time::Duration};

use crate::components::Component;


/// Trait that abstracts platform-specific functionality
pub trait Context {
    /// Sleep or delay for a duration.
    fn sleep(&self, duration: Duration);

    /// Report or log an error (like a failed poll).
    fn handle_error(&self, err: impl Error);

    /// Set up a rendering pass and start traversing component tree.
    fn render_setup(&self) -> Result<(), RenderError>;

    /// Quit the application.
    fn should_quit(&self) -> bool;
}

pub trait CanRender<C: Component> {
    fn render(
        &mut self, 
        component: &C,
    ) -> Result<(), RenderError>;
}

#[derive(Debug)]
pub struct RenderError;

// Implement Display for human-readable error messages
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Render error occurred")
    }
}

// Implement Error for interoperability with other error types
impl Error for RenderError {}

