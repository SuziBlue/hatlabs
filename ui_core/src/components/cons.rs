use leylines::ChannelState;

use super::Component;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cons<H: Component, T: Component> {
    pub head: H,
    pub tail: T,
}

impl<H: Component> Cons<H, Nil> {

    pub fn new(component: H) -> Cons<H, Nil> {
        Cons { head: component, tail: Nil }
    }
    pub fn push<C: Component>(self, component: C) -> Cons<H, Cons<C, Nil>> {
        Cons { 
            head: self.head, 
            tail: Cons {
                head: component,
                tail: Nil,
            }, 
        }
    }
}

impl<H: Component, T: Component> Component for Cons<H, T> {
    type State = ();

    fn poll(&mut self) -> Result<(), ChannelState> {
        self.head.poll()?;
        self.tail.poll()
    }
}

pub struct Nil;

impl Component for Nil {type State = ();}


