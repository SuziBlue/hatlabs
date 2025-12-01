
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ui_core::{comms, components::{self, ButtonClick, Nil, Text}, inputs::UserInputEvent, CanRender, RenderError};


use crate::input_handler::StdInputHandler;
use crate::layouts::RatatuiRect as RectRegion;

pub struct RatatuiFrame<'outer, 'inner> {
    pub frame: &'outer mut Frame<'inner>,
}

impl<'outer, 'inner, T> CanRender<components::TextBox<'_, T, RectRegion, u16>> for RatatuiFrame<'outer, 'inner>
where
    T: Text,
{
    fn render(&mut self, label: &components::TextBox<T, RectRegion, u16>) -> Result<(), RenderError> {
        let text = label.text.as_ref();
        let style = Style::default().fg(Color::White); // maybe from label.style?
        let block = Block::default().title("Label").borders(Borders::ALL);
        let paragraph = Paragraph::new(Span::styled(text, style)).block(block);
        self.frame.render_widget(paragraph, label.region.0);
        Ok(())
    }
}

impl<'outer, 'inner, S, R, T> CanRender<components::Button<'_, S, R, u16, i16, T, RectRegion>> for RatatuiFrame<'outer, 'inner>
where
    T: Text,
    S: comms::Sender<ButtonClick>,
    R: comms::Receiver<UserInputEvent<u16, i16>>,
{
    fn render(&mut self, button: &components::Button<S, R, u16, i16, T, RectRegion>) -> Result<(), RenderError> {
        let style = Style::default().fg(Color::Black).bg(Color::White);
        let block = Block::default().title("Button").borders(Borders::ALL);
        let paragraph = Paragraph::new(Span::styled(button.label.text.as_ref(), style)).block(block);
        self.frame.render_widget(paragraph, button.region.0);
        Ok(())
    }
}

impl<'outer, 'inner, H, T> CanRender<components::Cons<H, T>> for RatatuiFrame<'outer, 'inner>
where
    H: components::Component,
    T: components::Component,
    RatatuiFrame<'outer, 'inner>: CanRender<H> + CanRender<T>,
{
    fn render(&mut self, cons: &components::Cons<H, T>) -> Result<(), RenderError> {
        self.render(&cons.head)?;
        self.render(&cons.tail)
    }
}

impl<'outer, 'inner> CanRender<Nil> for RatatuiFrame<'outer, 'inner> {
    fn render(
            &mut self, 
            component: &Nil,
        ) -> Result<(), RenderError> {
        Ok(())
    }
}


impl<'outer, 'inner> CanRender<StdInputHandler> for RatatuiFrame<'outer, 'inner> {
    fn render(
            &mut self, 
            component: &StdInputHandler,
        ) -> Result<(), RenderError> {
        Ok(())
    }
}
