use std::{cmp::Reverse, collections::BinaryHeap, };

use futures::{future::pending, Stream, StreamExt};

use tokio::time::{Duration, Instant};
use tokio_stream::wrappers::ReceiverStream;


pub mod combinators;
pub mod heartbeat_manager;
pub mod reconnect_manager;



pub enum OutEvent<Out> {
    Output(
        Instant,
        Out,
    ),
    Timer(
        Duration
    ),
}

pub enum InEvent<In> {
    Input(
        Instant,
        In,
    ),
    Timeout(
        Instant
    )
}


pub trait TimedAutomata<In, Out> {

    type Fut: Future<Output = Vec<OutEvent<Out>>> + Send;

    fn on_event(
        &mut self,
        event: InEvent<In>
    ) -> Self::Fut;
}


pub enum Sided<A, B> {
    A(A),
    B(B),
}

pub trait BidirectionalTimedAutomata<InA, InB, OutA, OutB>: TimedAutomata<Sided<InA, InB>, Sided<OutA, OutB>> {}


pub struct Runtime {}



impl Runtime {
    pub fn run<I, O, P>(stream: impl Stream<Item = I> + Send + 'static, mut protocol: P) -> impl Stream<Item = O>
    where 
        P: TimedAutomata<I, O> + Send + 'static,
        I: Send + 'static,
        O: Send + 'static,
    {
        

        let (out_tx, out_rx) = tokio::sync::mpsc::channel(0);


        tokio::spawn(async move {
            tokio::pin!(stream);

            let mut timers = BinaryHeap::<Reverse<Instant>>::new();
            let mut next_timeout: Option<Instant> = None;

            loop {

                if let Some(Reverse(when)) = timers.peek() {
                    if next_timeout.is_none() {
                        next_timeout = Some(*when)
                    }
                }

                tokio::select! {
                    opt_input = stream.next() => {
                        if let Some(input) = opt_input {
                            let event = InEvent::Input(Instant::now(), input);
                            let out_events = protocol.on_event(event).await;
                            for out_event in out_events {
                                match out_event {
                                    OutEvent::Output(_, out) => {
                                        if let Err(_) = out_tx.send(out).await {
                                            break
                                        };
                                    },
                                    OutEvent::Timer(dur) => {
                                        timers.push(Reverse(Instant::now() + dur));
                                    },
                                }
                            }
                        }
                    }

                    _ = async {
                        if let Some(to) = &mut next_timeout {
                            tokio::time::sleep_until(*to).await;
                        } else {
                            pending::<()>().await
                        }
                    } => {
                        if let Some(Reverse(expired)) = timers.pop() {
                            let event = InEvent::Timeout(expired.into());
                            let out_events = protocol.on_event(event).await;

                            for out_event in out_events {
                                match out_event {
                                    OutEvent::Output(_, out) => {
                                        if let Err(_) = out_tx.send(out).await {
                                            break
                                        };
                                    },
                                    OutEvent::Timer(dur) => {
                                        timers.push(Reverse(Instant::now() + dur));
                                    },
                                }
                            }
                        }

                        next_timeout = None;
                    }
                }   
            }
        });


        ReceiverStream::new(out_rx)

    }
}
