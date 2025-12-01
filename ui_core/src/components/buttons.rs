use core::marker::PhantomData;

use leylines::{BroadcastSender, ChannelState, Receiver};

use crate::geometry::{Arithmetic, Region};

use super::{Component, Text, TextBox, TextStyle};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Button<
    'a, 
    S: BroadcastSender<ButtonClick>, 
    Pos: Arithmetic, 
    Dir: Arithmetic, 
    T: Text, 
    Reg: Region<Pos>,
> {
    pub label: TextBox<'a, T, Reg, Pos>,
    sender: S,
    pub region: &'a Reg,
    _phantom1: PhantomData<Pos>,
    _phantom2: PhantomData<Dir>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ButtonClick;


impl<'a, S, T, Pos, Dir, Reg> Button<'a, S, Pos, Dir, T, Reg>
where 
    S: BroadcastSender<ButtonClick>,
    T: Text,
    Pos: Arithmetic,
    Dir: Arithmetic,
    Reg: Region<Pos>,
{
    pub fn new(region: &'a Reg, sender: S, text: T, style: Option<TextStyle>) -> Self {
        Self {
            label: TextBox::new(text, region, style),
            sender,
            region,
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }
    pub fn click(&self) -> Result<(), ChannelState> {
        self.sender.try_send(ButtonClick)
    }
    pub fn subscribe(&self) -> impl Receiver<ButtonClick> {
        self.sender.subscribe()
    }
}

impl<S, T, Pos, Dir, Reg> Component for Button<'_, S, Pos, Dir, T, Reg> 
where 
    S: BroadcastSender<ButtonClick>,
    T: Text,
    Pos: Arithmetic,
    Dir: Arithmetic,
    Reg: Region<Pos>,
{
    type State = T;

    fn update(&mut self, new_state: Self::State) {
        self.label.text = new_state;
    }
    fn state(&self) -> Self::State {
        self.label.text.clone()
    }
}
