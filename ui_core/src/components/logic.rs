use core::marker::PhantomData;

use leylines::{ChannelState, Receiver, Sender};

use super::Component;


#[derive(Debug, Clone)]
pub struct Reactive<C, R>
where
    C: Component,
    R: Receiver<C::State>,
{
    pub component: C,
    pub receiver: R,
}

impl<C, R> Component for Reactive<C, R>
where
    C: Component,
    R: Receiver<C::State>,
{
    type State = C::State;

    fn state(&self) -> Self::State {
        self.component.state()
    }

    fn update(&mut self, new_state: Self::State) {
        self.component.update(new_state);
    }
    fn poll(&mut self) -> Result<(), ChannelState>{
        match self.receiver.try_recv() {
            Ok(msg) => {
                self.component.update(msg);
                Ok(())
            },
            Err(ChannelState::Closed) => Err(ChannelState::Closed),
            _ => Ok(()),
        } 
    }
}

impl<C, R> Reactive<C, R>
where
    C: Component,
    R: Receiver<C::State>,
{
    pub fn new(component: C, receiver: R) -> Self {
        Self { component, receiver }
    }
}


#[derive(Debug, Clone)]
pub struct Transform<A, B, R, W, S, F>
where
    R: Receiver<A>,
    W: Sender<B>,
    F: FnMut(&mut S, A) -> B,
    S: Clone,
{
    pub receiver: R,
    pub writer: W,
    pub state: S,
    pub transform: F,
    _phantom: PhantomData<(A, B)>,
}

impl<A, B, R, W, S, F> Transform<A, B, R, W, S, F>
where
    R: Receiver<A>,
    W: Sender<B>,
    F: FnMut(&mut S, A) -> B,
    S: Clone + Default,
{
    pub fn new(receiver: R, writer: W, state: S, transform: F) -> Self {
        Self { 
            receiver, 
            writer,
            state,
            transform,
            _phantom: PhantomData }
    }
}

impl<A, B, R, W, S, F> Component for Transform<A, B, R, W, S, F>
where
    R: Receiver<A>,
    W: Sender<B>,
    F: FnMut(&mut S, A) -> B,
    S: Clone + Default,
{
    type State = S;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn update(&mut self, new_state: Self::State) {
        self.state = new_state;
    }
    fn poll(&mut self) -> Result<(), ChannelState> {
        let msg = match self.receiver.try_recv() {
            Ok(msg) => msg,
            Err(ChannelState::Closed) => return Err(ChannelState::Closed),
            _ => return Ok(())
        };

        let output = (self.transform)(&mut self.state, msg);

        match self.writer.try_send(output) {
            Ok(()) => Ok(()),
            Err(ChannelState::Closed) => Err(ChannelState::Closed),
            _ => Ok(())
        }
    }
}
