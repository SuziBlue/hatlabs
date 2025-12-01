
use std::{task::Poll, time::Duration};

use futures::Stream;
use pin_project_lite::pin_project;
use tokio::time::{interval, Interval};


#[derive(Debug)]
pub struct Heartbeat {}

#[derive(Debug)]
pub struct HeartbeatAck {}

impl Heartbeat {
    pub fn new() -> Self {
        Heartbeat {  }
    }
}

#[derive(Debug)]
pub struct HeartbeatError {}

pub enum HeartbeatManagerInput {
    Heartbeat(Heartbeat),
    HeartbeatAck(HeartbeatAck),
}

pin_project! {
    pub struct HeartbeatManager<S> {
        #[pin]
        incoming: S,
        #[pin]
        interval: Interval,
        ack_received: bool,
    }
}

impl<S> HeartbeatManager<S> {
    pub fn new(incoming: S, heartbeat_interval: Duration) -> Self {
        HeartbeatManager { 
            incoming, 
            interval: interval(heartbeat_interval), 
            ack_received: true 
        }
    }
}

impl<S> Stream for HeartbeatManager<S> 
where 
    S: Stream<Item = HeartbeatManagerInput>,
{
    type Item = Result<Heartbeat, HeartbeatError>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            match this.incoming.as_mut().poll_next(cx) {
                Poll::Pending => break,
                Poll::Ready(Some(HeartbeatManagerInput::HeartbeatAck(_ack))) => {
                    *this.ack_received = true;
                    continue
                },
                Poll::Ready(Some(HeartbeatManagerInput::Heartbeat(_heartbeat))) => {
                    *this.ack_received = false;
                    this.interval.reset();
                    return Poll::Ready(Some(Ok(Heartbeat {  })))
                },
                Poll::Ready(None) => return Poll::Ready(None),
            };
        }

        match this.interval.poll_tick(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(_instant) => {
                if !*this.ack_received {
                    return Poll::Ready(Some(Err(HeartbeatError {})))
                } else {
                    *this.ack_received = false;
                    return Poll::Ready(Some(Ok(Heartbeat {})))
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use tokio::time::{advance, sleep, Duration};
    use tokio_stream::wrappers::ReceiverStream;

    #[tokio::test(start_paused = true)] // <-- IMPORTANT: we control time
    async fn test_heartbeat_manager_emits_heartbeats_and_handles_acks() {
        // incoming acks (empty at first)
        let (ack_tx, ack_rx) = tokio::sync::mpsc::channel(10);
        let incoming_stream = ReceiverStream::new(ack_rx);

        // create the heartbeat stream
        let mut hb_stream = HeartbeatManager::new(incoming_stream, Duration::from_secs(1));

        advance(Duration::from_millis(1)).await;
        let first = hb_stream.next().await.expect("expected heartbeat");
        println!("First: {:?}", &first);
        assert!(first.is_ok(), "first event should be Ok(Heartbeat)");

        //
        // ---- Send an ack BEFORE the next timeout ----
        //
        ack_tx.send(HeartbeatManagerInput::HeartbeatAck(HeartbeatAck {  })).await.unwrap();

        // Tick the clock so the next heartbeat is generated
        advance(Duration::from_secs(1)).await;

        let second = hb_stream.next().await.expect("expected second heartbeat");
        println!("Second: {:?}", &second);
        assert!(second.is_ok(), "second event should also be Ok(Heartbeat)");

        // Send heartbeat so the next heartbeat is generated early
        advance(Duration::from_millis(200)).await;
        ack_tx.send(HeartbeatManagerInput::Heartbeat(Heartbeat {  })).await.unwrap();

        let third = hb_stream.next().await.expect("expected third heartbeat");
        println!("Third: {:?}", &third);
        assert!(third.is_ok(), "third event should also be Ok(Heartbeat)");

        //
        // ---- Advance time long enough for a timeout event ----
        //
        advance(Duration::from_secs(2)).await;

        let fourth = hb_stream.next().await.expect("expected timeout event");
        println!("Fourth: {:?}", &fourth);
        assert!(fourth.is_err(), "expected Err(HeartbeatError)");
    }
}


