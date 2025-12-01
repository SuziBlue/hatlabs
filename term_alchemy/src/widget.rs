use std::error::Error;

pub trait Widget<ES, S, R>
where
    ES: EventStream,
    S: State,
    R: Renderer,
{
    fn handle_event(&mut self);
}

pub trait Event {
    fn handle(self);
}

pub trait State {
    fn handle_event(&mut self, event: impl Event) -> impl State;
}

pub trait Renderer {
    fn render(&self, state: &impl State);
}

pub trait EventStream {
    type E: Event;
    fn next(&self) -> Self::E;
}
