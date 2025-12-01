use std::{error::Error, fmt::Display, io};

use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    state: AppState,
}

#[derive(Debug, PartialEq, Eq)]
enum AppState {
    Running,
    Quitting,
}
impl Default for AppState {
    fn default() -> Self {
        AppState::Running
    }
}

impl App {
    pub fn run(mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        while self.state != AppState::Quitting {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        ratatui::restore();
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        todo!()
    }
    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
