use leylines::{ChannelState, Receiver};

use crate::traits::{CanRender, RenderError};

use super::{Cons, Nil, Reactive};


pub trait Component: Sized {
    type State: Clone + Default;

    fn reactive<R: Receiver<Self::State>>(self, receiver: R) -> Reactive<Self, R> {
        Reactive {
            component: self,
            receiver,
        }
    }
    fn push<C: Component>(self, component: C) -> Cons<Self, Cons<C, Nil>> {
        Cons::new(self)
            .push(component)
    }
    fn state(&self) -> Self::State {
        Self::State::default()
    }
    fn update(&mut self, new_state: Self::State) {}
    fn poll(&mut self) -> Result<(), ChannelState> {
        Ok(())
    }
    fn render(&mut self, ctx: &mut impl CanRender<Self>) -> Result<(), RenderError> {
        ctx.render(self)
    }
}
