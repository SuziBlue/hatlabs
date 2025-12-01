use std::{collections::VecDeque, pin::Pin, task::Poll};

use futures::{future::Either, stream::{unfold, Peekable}, Sink, SinkExt, Stream, StreamExt};
use pin_project_lite::pin_project;






pub fn reconnect_manager<C, State, S, Fut, Item>(connect_protocol: C, initial_state: State) -> impl Stream<Item = Item> 
where 
    S: Stream<Item = Item>,
    C: FnMut(State) -> Fut,
    Fut: Future<Output = Option<(S, State)>>
{
    futures::stream::unfold(initial_state, connect_protocol).flatten()
}


pin_project! {
    pub struct ReconnectManager<Inner, Item, SendFn, SendFut, RecvFn, RecvFut> 
    {
        inner: Option<Inner>,

        on_send: SendFn, 
        #[pin]
        send_future: Option<SendFut>,
        send_queue: VecDeque<Item>,

        on_recv: RecvFn, 
        #[pin]
        recv_future: Option<RecvFut>,

    }
}

impl<Inner, Item, E, SendFn, SendFut, RecvFn, RecvFut> Stream for ReconnectManager<Inner, Item, SendFn, SendFut, RecvFn, RecvFut> 
where 
    Inner: Sink<Item> + Stream,
    SendFn: FnMut(Inner, Item) -> SendFut,
    SendFut: Future<Output = Result<Inner, E>>,
    RecvFn: FnMut(Inner) -> RecvFut,
    RecvFut: Future<Output = Option<(Inner, Item)>>,
{
    type Item = Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            
            if let Some(fut) = this.recv_future.as_mut().as_pin_mut() {
                match fut.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Some((new_inner, item))) => {
                        this.recv_future.set(None);
                        *this.inner = Some(new_inner);
                        return Poll::Ready(Some(item));
                    }
                    Poll::Ready(None) => {
                        this.recv_future.set(None);
                        return Poll::Ready(None);
                    }
                }
            }

            if let Some(inner) = this.inner.take() {
                this.recv_future.set(Some((this.on_recv)(inner)));
                continue
            } else {
                return Poll::Pending
            }
        }
    }
}

impl<Inner, Item, E, SendFn, SendFut, RecvFn, RecvFut> Sink<Item> for ReconnectManager<Inner, Item, SendFn, SendFut, RecvFn, RecvFut> 
where 
    Inner: Sink<Item> + Stream,
    SendFn: FnMut(Inner, Item) -> SendFut,
    SendFut: Future<Output = Result<Inner, E>>,
    RecvFn: FnMut(Inner) -> RecvFut,
    RecvFut: Future<Output = Option<(Inner, Item)>>,
{
    type Error = E;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        let this = self.project();

        this.send_queue.push_back(item);

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        loop {
            if let Some(fut) = this.send_future.as_mut().as_pin_mut() {
                match fut.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Ok(inner)) => {
                        this.send_future.set(None);
                        *this.inner = Some(inner);
                        return Poll::Ready(Ok(()));
                    },
                    Poll::Ready(Err(e)) => {
                        this.send_future.set(None);
                        *this.inner = None;
                        return Poll::Ready(Err(e));
                    }
                }
            }

            if let Some(inner) = this.inner.take() {
                match this.send_queue.pop_front() {
                    Some(item) => {
                        this.send_future.set(Some((this.on_send)(inner, item)));
                        continue;
                    }
                    None => {
                        return Poll::Ready(Ok(()));
                    }
                }
            } else {
                return Poll::Pending;
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_flush(cx)
    }
}



pin_project! {
    pub struct FailoverSink<S, I> 
    where 
        S: Stream
    {
        #[pin]
        sinks: Peekable<S>,
        current: Option<S::Item>,
        queue: VecDeque<I>,
        pending: VecDeque<I>,
    }
}

#[derive(Debug)]
pub struct FailoverSinkError {}


impl<S, Si, I> FailoverSink<S, I> 
where 
    S: Stream<Item = Si>,
    Si: Sink<I>,
    I: Clone,
{
    pub fn new(sinks: S) -> Self 
    {
        Self { 
            sinks: sinks.peekable(), 
            current: None, 
            queue: VecDeque::new(),
            pending: VecDeque::new(),
        }
    }
}

impl<S, I> Sink<I> for FailoverSink<S, I> 
where 
    S: Stream,
    S::Item: Sink<I> + Unpin,
    I: Clone,
{
    type Error = FailoverSinkError;

    fn poll_ready(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.as_mut().project();

        match this.current {
            Some(_sink) => Poll::Ready(Ok(())),
            None => {
                match this.sinks.poll_peek(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Some(_)) => Poll::Ready(Ok(())),
                    Poll::Ready(None) => Poll::Ready(Err(FailoverSinkError {  })),
                }
            }
        }
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        let this = self.project();

        this.queue.push_back(item);

        Ok(())
        
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        loop {
            // Ensure we have a sink
            if this.current.is_none() {
                match this.sinks.as_mut().poll_next(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Some(next_sink)) => {
                        *this.current = Some(next_sink);
                        // Move pending items back into queue for retry
                        while let Some(p) = this.pending.pop_back() {
                            this.queue.push_front(p);
                        }
                    }
                    Poll::Ready(None) => {
                        return Poll::Ready(Err(FailoverSinkError {}));
                    }
                }
            }

            // Now we have a current sink
            let mut sink = this.current.take().unwrap();

            // Must pin current sink
            let mut pin_sink = Pin::new(&mut sink);

            // Wait for sink to be ready
            match pin_sink.as_mut().poll_ready(cx) {
                Poll::Pending => {
                    *this.current = Some(sink);
                    return Poll::Pending
                }
                Poll::Ready(Err(_)) => {
                    // Sink failed; drop it and try next
                    *this.current = None;
                    continue;
                }
                Poll::Ready(Ok(())) => {
                    // Sink ready; push items
                    while let Some(item) = this.queue.pop_front() {
                        this.pending.push_back(item.clone());

                        match pin_sink.as_mut().start_send(item) {
                            Ok(()) => continue,
                            Err(_) => {
                                // Sink broke during send
                                *this.current = None;
                                break;
                            }
                        }
                    }

                    // If queue empty, flush inner sink
                    if this.queue.is_empty() {
                        match pin_sink.as_mut().poll_flush(cx) {
                            Poll::Pending => {
                                *this.current = Some(sink);
                                return Poll::Pending;
                            }
                            Poll::Ready(Err(_)) => {
                                *this.current = None;
                                continue;
                            }
                            Poll::Ready(Ok(())) => {
                                this.pending.clear();
                                *this.current = Some(sink);
                                return Poll::Ready(Ok(()));
                            }
                        }
                    }
                }
            }
        }
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.project();

        if let Some(sink) = this.current {
            match Pin::new(sink).poll_close(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
                Poll::Ready(Err(_)) => Poll::Ready(Err(FailoverSinkError {  }))
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }
}


#[cfg(test)]
mod tests{

    use std::{collections::VecDeque, pin::Pin, task::{Context, Poll}};
    use futures::Sink;

    use futures::{stream, SinkExt, Stream, StreamExt};
    use tokio;
    use tokio_util::sync::PollSender;

    use crate::reconnect_manager::{FailoverSink, ReconnectManager};
    #[tokio::test]
    async fn test_failover_sink() {
        let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(10);
        let (tx2, mut rx2) = tokio::sync::mpsc::channel::<i32>(10);

        let sinks = stream::iter(vec![PollSender::new(tx1), PollSender::new(tx2)]);

        let mut failover_sink = FailoverSink::new(sinks);

        failover_sink.send(10).await.unwrap();
        assert_eq!(rx1.recv().await.unwrap(), 10);
        drop(rx1);

        failover_sink.send(20).await.unwrap();
        assert_eq!(rx2.recv().await.unwrap(), 20);
        drop(rx2);

        assert!(failover_sink.send(30).await.is_err());
    }

    // A simple mock Inner that implements Sink + Stream
    #[derive(Debug, Clone)]
    struct MockInner {
        sent: Vec<i32>,
        to_recv: VecDeque<i32>,
    }

    impl Sink<i32> for MockInner {
        type Error = ();

        fn poll_ready(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn start_send(self: Pin<&mut Self>, item: i32) -> Result<(), Self::Error> {
            let this = self.get_mut();
            this.sent.push(item);
            Ok(())
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
    }

    impl Stream for MockInner {
        type Item = i32;

        fn poll_next(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>
        ) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.to_recv.pop_front())
        }
    }

    #[tokio::test]
    async fn reconnect_manager_basic() {

        let inner = MockInner {
            sent: vec![],
            to_recv: VecDeque::from(vec![10, 20, 30, 1, 1, 1]),
        };

        // on_send just returns a ready future that returns inner
        let on_send = |mut inner: MockInner, item: i32| {
            async move {
                let _ = inner.send(item).await;
                Ok(inner)
            }
        };

        // on_recv just returns a ready future that pops one item
        let on_recv = |mut inner: MockInner| {
            async move {
                let item = inner.next().await;
                item.map(|i| (inner, i))
            }
        };
        
        let mgr = ReconnectManager {
            inner: Some(inner),
            on_send,
            send_future: None,
            send_queue: VecDeque::new(),
            on_recv,
            recv_future: None,
        };

        futures::pin_mut!(mgr);

        // Test sending items
        let _: Result<(), ()> = mgr.send(1).await;
        let _: Result<(), ()> = mgr.send(2).await;

        // Receive items from the stream
        let received: Vec<i32> = mgr.as_mut().take(3).collect().await;

        // Check results
        assert_eq!(received, vec![10, 20, 30]);
        // The inner should have received the sent items
        let inner = mgr.inner.clone().unwrap();
        assert_eq!(inner.sent, vec![1, 2]);
    }
}
