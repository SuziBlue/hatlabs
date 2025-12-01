use crossterm::event::{self, Event};
use oauth2::{StandardTokenResponse, TokenResponse};
use ratatui::{DefaultTerminal, Frame};
use std::io::Error;
use youtube::YoutubeToken;

pub mod app;
pub mod widget;
pub mod youtube;

pub fn run(mut terminal: DefaultTerminal) -> Result<(), Error> {
    let token = youtube::get_youtube_token();
    loop {
        terminal.draw(|frame| {
            render(frame, token.clone());
        })?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, token: YoutubeToken) {
    frame.render_widget("Hello world", frame.area());
}
