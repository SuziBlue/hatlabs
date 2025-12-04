
use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet, VecDeque}, fmt::Debug, marker::PhantomData, pin::Pin, task::Poll};

use futures::{future::Either, stream::Peekable, FutureExt, Sink, Stream, StreamExt};
use pin_project_lite::pin_project;
use tokio::{sync::mpsc::channel, time::{sleep_until, Sleep}, time::{Duration, Instant}};
use tokio_stream::wrappers::ReceiverStream;








#[derive(Debug)]
pub struct Timed<T> {
    pub value: T,
    pub timestamp: Instant,
}

impl<T> Timed<T> {
    pub fn new(value: T, timestamp: Instant) -> Self {
        Timed { value, timestamp }
    }
    pub fn tag_now(value: T) -> Self {
        Timed { value, timestamp: Instant::now() }
    }
}

impl<T> PartialEq for Timed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}
impl<T> Eq for Timed<T> {}

impl<T> PartialOrd for Timed<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl<T> Ord for Timed<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

pub struct Event<T> {
    pub value: Timed<T>,
    pub id: Id,
}

impl<T> Event<T> {
    pub fn new(value: T, when: Instant, id: Id) -> Self {
        Event { 
            value: Timed::new(value, when), 
            id 
        }
    }
    pub fn event_now(value: T, id: Id) -> Self {
        Event { 
            value: Timed::tag_now(value), 
            id 
        }
    }
    pub fn event_in(value: T, duration: Duration, id: Id) -> Self {
        Event::new(value, Instant::now() + duration, id)
    }
}

impl<T> PartialEq for Event<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T> Eq for Event<T> {}

impl<T> PartialOrd for Event<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl<T> Ord for Event<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

pub struct Traced<F, X, Y, U> 
where 
    F: Fn(X, U) -> (Y, U),
{
    f: F,
    state: Option<U>,
    _marker: PhantomData<(X, Y)>
}

impl<F, X, Y, U> Traced<F, X, Y, U>
where 
    F: Fn(X, U) -> (Y, U),
{
    pub fn new(f: F, u_0: U) -> impl FnMut(X) -> Y {
        let mut traced = Traced {
            f,
            state: Some(u_0),
            _marker: PhantomData
        };
        move |x| {
            traced.call(x)
        }
    }

    fn call(&mut self, x: X) -> Y {
        let f = &self.f;

        let state = self.state.take().expect("Must have state");

        let (y, new_state) = f(x, state);

        self.state = Some(new_state);

        return y
    }
}



pin_project! {
    pub struct Wrapper<F, S, X, Y> 
    where 
        F: FnMut(X) -> Y
    {
        #[pin]
        inner: S,
        f: F,
        _marker: PhantomData<(X, Y)>
    }
}

impl<F, S, X, Y> Stream for Wrapper<F, S, X, Y> 
where 
    F: FnMut(X) -> Y,
    S: Stream<Item = X>,
{
    type Item = Y;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();

        let inner_item = match this.inner.poll_next(cx) {
            std::task::Poll::Ready(Some(item)) => item,
            std::task::Poll::Ready(None) => return std::task::Poll::Ready(None),
            std::task::Poll::Pending => return std::task::Poll::Pending,
        };

        let next_item = (this.f)(inner_item);

        std::task::Poll::Ready(Some(next_item))
    }
}











pin_project! {
    pub struct MergeSorted<A, B> where
        A: Stream,
        B: Stream<Item = A::Item>,
        A::Item: Ord,
    {
        #[pin]
        a: Peekable<A>,
        #[pin]
        b: Peekable<B>,
    }
}

pub fn merge_sort<A, B>(a: A, b: B) -> MergeSorted<A, B> where 
    A: Stream,
    B: Stream<Item = A::Item>,
    A::Item: Ord + Debug,
{
    MergeSorted { 
        a: a.peekable(), 
        b: b.peekable(), 
    }
}

impl<A, B> Stream for MergeSorted<A, B>
where 
    A: Stream,
    B: Stream<Item = A::Item>,
    A::Item: Ord + Debug,
{
    type Item = A::Item;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();


        match (this.a.as_mut().poll_peek(cx), this.b.as_mut().poll_peek(cx)) {
            (Poll::Pending, _) => Poll::Pending,
            (_, Poll::Pending) => Poll::Pending,
            (Poll::Ready(Some(a_item)), Poll::Ready(Some(b_item))) => {
                if a_item <= b_item {
                    this.a.poll_next(cx)
                } else {
                    this.b.poll_next(cx)
                }
            }
            (Poll::Ready(Some(_a_item)), Poll::Ready(None)) => this.a.poll_next(cx),
            (Poll::Ready(None), Poll::Ready(Some(_b_item))) => this.b.poll_next(cx),
            (Poll::Ready(None), Poll::Ready(None)) => Poll::Ready(None)
        }


    }
}





type Id = usize;

pub enum ScheduleCommand<T> {
    Schedule(Event<T>),
    Cancel(Id),
}

pin_project! {
    pub struct Scheduler<St, T>
    where 
        St: Stream<Item = ScheduleCommand<T>>
    {
        heap: BinaryHeap<Reverse<Event<T>>>,
        cancelled: HashSet<Id>,
        #[pin]
        inner: St,
        #[pin]
        sleep: Option<Pin<Box<Sleep>>>,
    }
}


impl<St, T> Scheduler<St, T>
where 
    St: Stream<Item = ScheduleCommand<T>>
{
    pub fn new(inner: St) -> Self {
        Scheduler { 
            heap: BinaryHeap::new(), 
            cancelled: HashSet::new(), 
            inner,
            sleep: None,
        }
    }
}



impl<St, T> Stream for Scheduler<St, T> 
where 
    St: Stream<Item = ScheduleCommand<T>>,
{
    type Item = Event<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            while let Poll::Ready(Some(command)) = this.inner.as_mut().poll_next(cx) {
                match command {
                    ScheduleCommand::Schedule(event) => {
                        this.heap.push(Reverse(event));
                    }
                    ScheduleCommand::Cancel(id) => {
                        this.cancelled.insert(id);
                    }
                }
            }

            while let Some(Reverse(next_event)) = this.heap.peek() {
                if this.cancelled.remove(&next_event.id) {
                    this.heap.pop();
                    continue
                } 
                let when = next_event.value.timestamp;
                if when <= Instant::now() {
                    let Reverse(event) = this.heap.pop().unwrap();
                    return Poll::Ready(Some(event))
                } else {
                    *this.sleep = Some(Box::pin(sleep_until(when)));
                    break
                }
            };


            if let Some(mut sleep) = this.sleep.as_mut().as_pin_mut() {
                match sleep.poll_unpin(cx) {
                    Poll::Ready(_) => {
                        *this.sleep = None;
                        continue
                    },
                    Poll::Pending => return Poll::Pending
                }
            } else {
                return Poll::Pending
            }
        }
    }
}


pub trait StreamExtSplit: Stream + Sized {

    fn split_either<L, R, S>(mut self) -> (ReceiverStream<L>, ReceiverStream<R>) 
    where 
        Self: Stream<Item = Either<L, R>> + Unpin + Send + 'static,
        L: Unpin + Send + 'static,
        R: Unpin + Send + 'static,
    {
        let (left_tx, left_rx) = channel(1);
        let (right_tx, right_rx) = channel(1);

        tokio::spawn(async move {
            while let Some(item) = self.next().await {
                match item {
                    Either::Left(l) => {
                        let _ = left_tx.send(l).await;
                    },
                    Either::Right(r) => {
                        let _ = right_tx.send(r).await;
                    },
                }
            }
        });

        (ReceiverStream::new(left_rx), ReceiverStream::new(right_rx))
    }


    fn filter_split<P, I>(self, predicate: P) -> (ReceiverStream<I>, ReceiverStream<I>) 
    where 
        Self: Stream<Item = I> + Send + 'static,
        P: Fn(&I) -> bool + Send + 'static,
        I: Send + 'static
    {
        let (true_tx, true_rx) = channel(1);
        let (false_tx, false_rx) = channel(1);

        tokio::spawn(async move {
            let stream = self;
            tokio::pin!(stream);
            while let Some(item) = stream.next().await {
                if predicate(&item) {
                    let _ = true_tx.send(item).await;
                } else {
                    let _ = false_tx.send(item).await;
                }
            }
        });

        (ReceiverStream::new(true_rx), ReceiverStream::new(false_rx))
    }

    fn filter_either<P, I>(self, predicate: P) -> impl Stream<Item = Either<I, I>> 
    where
        Self: Stream<Item = I>,
        P: Fn(&I) -> bool,
    {
        self.map(move |item| {
            if predicate(&item) {
                Either::Left(item)
            } else {
                Either::Right(item)
            }
        })
    }
}

impl<T> StreamExtSplit for T where T: Stream + Sized {}

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

impl<Inner, Item, SendFn, SendFut, RecvFn, RecvFut> ReconnectManager<Inner, Item, SendFn, SendFut, RecvFn, RecvFut> {
    pub fn new(inner: Inner, on_send: SendFn, on_recv: RecvFn) -> Self {
        Self { 
            inner: Some(inner), 
            on_send, 
            send_future: None, 
            send_queue: VecDeque::new(), 
            on_recv, 
            recv_future: None 
        }
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

    use futures::{future::Either, stream::{self, Iter, StreamExt}};
    use tokio::time::{sleep, timeout, Instant, Duration};

    use crate::combinators::{merge_sort, Event, ScheduleCommand, Scheduler, Timed, StreamExtSplit};
    
    #[tokio::test]
    async fn test_traced_scan() {
        let s = futures::stream::iter([1, 2, 3]);

        let traced = s.scan(0, |state, x| {
            let y = x + *state;
            *state += 1;

            async move { Some(y) }
        });

        let out: Vec<_> = traced.collect().await;

        assert_eq!(out, vec![1, 3, 5]);
    }



    #[tokio::test]
    async fn test_merge_sorted_timed() {
        let now = Instant::now();

        let s1 = stream::iter(vec![
            Timed { value: "a", timestamp: now + Duration::from_secs(1) },
            Timed { value: "b", timestamp: now + Duration::from_secs(3) },
        ]);

        let s2 = stream::iter(vec![
            Timed { value: "c", timestamp: now + Duration::from_secs(2) },
            Timed { value: "d", timestamp: now + Duration::from_secs(4) },
        ]);

        let merged = merge_sort(s1, s2);

        let out: Vec<_> = merged.collect().await;

        // Extract values for easier assertion
        let values: Vec<_> = out.iter().map(|t| t.value).collect();

        assert_eq!(values, vec!["a", "c", "b", "d"]);

        // Assert timestamps are sorted
        for i in 1..out.len() {
            assert!(out[i-1].timestamp <= out[i].timestamp);
        }
    }
    #[tokio::test]
    async fn scheduler_emits_events_in_order_and_respects_cancellation() {
        tokio::time::pause();

        // === define commands for the scheduler ===
        let cmds = vec![
            ScheduleCommand::Schedule(Event::event_in("first", Duration::from_millis(50), 1)),
            ScheduleCommand::Schedule(Event::event_in("second", Duration::from_millis(10), 2)),
            ScheduleCommand::Schedule(Event::event_in("third", Duration::from_millis(30), 3)),
            ScheduleCommand::Cancel(3), // cancel event #3 before it fires
        ];

        let stream = stream::iter(cmds);
        let mut sched = Scheduler::new(stream);

        // === advance time just enough for event 2 ===
        tokio::time::advance(Duration::from_millis(10)).await;
        assert_eq!(sched.next().await.unwrap().value.value, "second");

        // === next should be event 1 after 50 ms total ===
        tokio::time::advance(Duration::from_millis(40)).await;
        assert_eq!(sched.next().await.unwrap().value.value, "first");

        // === event 3 was cancelled, so scheduler should now be Pending ===
        tokio::time::advance(Duration::from_millis(100)).await;
        tokio::select! {
            biased;
            _ = sched.next() => panic!("Cancelled event fired!"),
            _ = sleep(Duration::from_millis(10)) => {} // ok, no more events
        }
    }

    #[tokio::test]
    async fn test_split_either() {
        // Create a stream of mixed Either<L, R> values
        let input_stream = stream::iter(vec![
            Either::Left(1),
            Either::Right("a"),
            Either::Left(2),
            Either::Right("b"),
        ]);

        let (mut left_rx, mut right_rx) = input_stream.split_either::<i32, &str, Iter<Either<i32, &str>>>();

        // --- Assert left values ---
        let l1 = timeout(Duration::from_millis(100), left_rx.next())
            .await
            .expect("left recv timeout")
            .expect("left channel closed");
        assert_eq!(l1, 1);

        // --- Assert right values ---
        let r1 = timeout(Duration::from_millis(100), right_rx.next())
            .await
            .expect("right recv timeout")
            .expect("right channel closed");
        assert_eq!(r1, "a");

        let l2 = timeout(Duration::from_millis(100), left_rx.next())
            .await
            .expect("left recv timeout")
            .expect("left channel closed");
        assert_eq!(l2, 2);

        // Make sure no more left values
        assert!(timeout(Duration::from_millis(50), left_rx.next())
            .await
            .expect("left recv timeout")
            .is_none());

        let r2 = timeout(Duration::from_millis(100), right_rx.next())
            .await
            .expect("right recv timeout")
            .expect("right channel closed");
        assert_eq!(r2, "b");

        // No more right values
        assert!(timeout(Duration::from_millis(50), right_rx.next())
            .await
            .expect("right channel closed")
            .is_none());
    }    

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


