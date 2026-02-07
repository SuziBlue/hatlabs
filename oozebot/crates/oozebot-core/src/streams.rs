
use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet, VecDeque}, fmt::Debug, marker::PhantomData, mem, pin::Pin, sync::{atomic::AtomicUsize, Arc}, task::{ready, Context, Poll}};

use either::Either;
use futures::{lock::{Mutex, OwnedMutexGuard, OwnedMutexLockFuture}, stream::Peekable, FutureExt, Sink, SinkExt, Stream, StreamExt};
use pin_project_lite::pin_project;
use tokio::{sync::mpsc::channel, time::{sleep_until, Duration, Instant, Sleep}};
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

    fn split_either<L, R>(mut self) -> (ReceiverStream<L>, ReceiverStream<R>) 
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
    pub struct ReconnectManager<Inner, SinkItem, StreamItem, E> 
    {
        inner: Arc<Mutex<Inner>>,

        on_send: Box<dyn FnMut(Arc<Mutex<Inner>>, SinkItem) -> Pin<Box<dyn Future<Output = Result<(), E>>>>>,
        #[pin]
        send_future: Option<Pin<Box<dyn Future<Output = Result<(), E>>>>>,
        send_queue: VecDeque<SinkItem>,

        on_recv: Box<dyn FnMut(Arc<Mutex<Inner>>) -> Pin<Box<dyn Future<Output = Option<StreamItem>>>>>, 
        #[pin]
        recv_future: Option<Pin<Box<dyn Future<Output = Option<StreamItem>>>>>,

    }
}

impl<Inner, SinkItem, StreamItem, E> ReconnectManager<Inner, SinkItem, StreamItem, E> {
    pub fn new(
        inner: Inner, 
        on_send: impl (FnMut(Arc<Mutex<Inner>>, SinkItem) -> Pin<Box<dyn Future<Output = Result<(), E>>>>) + 'static, 
        on_recv: impl (FnMut(Arc<Mutex<Inner>>) -> Pin<Box<dyn Future<Output = Option<StreamItem>>>>) + 'static
    ) -> Self 
    {
        Self { 
            inner: Arc::new(Mutex::new(inner)),
            on_send: Box::new(on_send), 
            send_future: None, 
            send_queue: VecDeque::new(), 
            on_recv: Box::new(on_recv), 
            recv_future: None 
        }
    }
}

impl<Inner, SinkItem, StreamItem, E> Stream for ReconnectManager<Inner, SinkItem, StreamItem, E> 
where 
    Inner: Stream,
{
    type Item = StreamItem;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            
            if let Some(fut) = this.recv_future.as_mut().as_pin_mut() {
                match fut.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Some(item)) => {
                        this.recv_future.set(None);
                        return Poll::Ready(Some(item));
                    }
                    Poll::Ready(None) => {
                        this.recv_future.set(None);
                        return Poll::Ready(None);
                    }
                }
            } else {
                this.recv_future.set(Some((this.on_recv)(this.inner.clone())));
            }
        }
    }
}

impl<Inner, SinkItem, StreamItem, E> Sink<SinkItem> for ReconnectManager<Inner, SinkItem, StreamItem, E> 
where 
    Inner: Sink<SinkItem>,
{
    type Error = E;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: SinkItem) -> Result<(), Self::Error> {
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
                    Poll::Ready(Ok(())) => {
                        this.send_future.set(None);
                        return Poll::Ready(Ok(()));
                    },
                    Poll::Ready(Err(e)) => {
                        this.send_future.set(None);
                        return Poll::Ready(Err(e));
                    }
                }
            }

            match this.send_queue.pop_front() {
                Some(item) => {
                    this.send_future.set(Some((this.on_send)(this.inner.clone(), item)));
                    continue;
                }
                None => {
                    return Poll::Ready(Ok(()));
                }
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

pin_project! {
    pub struct Duplex<Si, St> {
        #[pin]
        sink: Si,
        #[pin]
        stream: St,
    }
}

impl<Si, St> Duplex<Si, St> {
    pub fn new(sink: Si, stream: St) -> Self {
        Duplex { sink, stream }
    }
}

impl<Si, St, Item> Sink<Item> for Duplex<Si, St>
where 
    Si: Sink<Item>,
{
    type Error = Si::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sink.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().sink.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sink.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sink.poll_close(cx)
    }
}

impl<Si, St> Stream for Duplex<Si, St>
where 
    St: Stream,
{
    type Item = St::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

enum FanInSinkState {
    Open,
    Closing,
    Closed,
}

pin_project! {
    pub struct FanInSink<Si, Item> 
    {
        inner_sink: Arc<Mutex<Si>>,
        open_peers: AtomicUsize,
        state: FanInSinkState,
        #[pin]
        lock: Option<Either<OwnedMutexLockFuture<Si>, OwnedMutexGuard<Si>>>,
        _marker: PhantomData<Item>,
    }
}

impl<Si, Item> FanInSink<Si, Item> 
where 
    Si: Sink<Item>
{
    pub fn new(sink: Si) -> Self {
        FanInSink { 
            inner_sink: Arc::new(Mutex::new(sink)), 
            open_peers: AtomicUsize::new(1),
            state: FanInSinkState::Open,
            lock: None, 
            _marker: PhantomData }
    }

    fn acquire_lock(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<OwnedMutexGuard<Si>> {
        let mut this = self.project();

        loop {
            match this.lock.take() {
                Some(Either::Left(mut fut)) => {
                    match fut.poll_unpin(cx) {
                        Poll::Pending => {
                            *this.lock = Some(Either::Left(fut));
                            return Poll::Pending
                        }
                        Poll::Ready(sink) => {
                            *this.lock = Some(Either::Right(sink));
                            continue
                        },
                    }
                },
                Some(Either::Right(sink)) => return Poll::Ready(sink),
                None => {
                    *this.lock = Some(Either::Left(this.inner_sink.clone().lock_owned()));
                    continue
                },
            };
        }
    }

    fn return_sink(self: Pin<&mut Self>, sink: OwnedMutexGuard<Si>) {
        let mut this = self.project();

        *this.lock = Some(Either::Right(sink));
    }
}

impl<Si, Item> Clone for FanInSink<Si, Item> 
{
    fn clone(&self) -> Self {
        let open_peers = self.open_peers.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;

        FanInSink { 
            inner_sink: self.inner_sink.clone(), 
            open_peers: open_peers.into(),
            state: FanInSinkState::Open,
            lock: None, 
            _marker: PhantomData }
    }
}

impl<Si, Item> Sink<Item> for FanInSink<Si, Item> 
where 
    Si: Sink<Item> + Unpin
{
    type Error = Si::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {

        let mut sink = ready!(self.as_mut().acquire_lock(cx));
        let poll = sink.poll_ready_unpin(cx);

        self.return_sink(sink);

        return poll
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        let mut this = self.project();

        match this.lock.take() {
            Some(Either::Right(mut sink)) => {
                let res = sink.start_send_unpin(item);
                *this.lock = Some(Either::Right(sink));
                return res
            }
            _ => panic!("Should not call start_send before poll_ready")
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut this = self.project();

        match this.lock.take() {
            Some(Either::Right(mut sink)) => {
                match sink.poll_flush_unpin(cx) {
                    Poll::Pending => {
                        *this.lock = Some(Either::Right(sink));
                        return Poll::Pending
                    }
                    Poll::Ready(Ok(())) => {
                        // Sink is finished flushing so drop the guard.
                        drop(sink);
                        return Poll::Ready(Ok(()));
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e))
                };
            }
            _ => panic!("Should not call poll_flush before poll_ready")
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {

        loop {
            match self.state {
                FanInSinkState::Open => {
                    self.state = FanInSinkState::Closing;
                    if self.open_peers.fetch_sub(1, std::sync::atomic::Ordering::AcqRel) > 1 {
                        *self.as_mut().project().lock = None;
                        self.state = FanInSinkState::Closed;
                        return Poll::Ready(Ok(()));
                    }
                }
                FanInSinkState::Closing => {
                    let mut sink = ready!(self.as_mut().acquire_lock(cx));
                    match sink.poll_close_unpin(cx) {
                        Poll::Pending => {
                            self.as_mut().return_sink(sink);
                            return Poll::Pending;
                        },
                        Poll::Ready(Ok(())) => {
                            self.state = FanInSinkState::Closed;
                            return Poll::Ready(Ok(()))
                        },
                        Poll::Ready(Err(e)) => {
                            self.state = FanInSinkState::Closed;
                            return Poll::Ready(Err(e))
                        }
                    }
                }
                FanInSinkState::Closed => {
                    return Poll::Ready(Ok(()))
                }
            }
        }
    }
}

#[pin_project::pin_project(project = CachedFutureProj)]
pub enum CachedFuture<F: Future> {
    Fut(#[pin] F),
    Avail(F::Output),
}

impl<'a, F> Future for CachedFuture<F> 
where 
    F: Future,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.as_mut().project();
        match this {
            CachedFutureProj::Fut(fut) => {
                let inner = ready!(fut.poll(cx));
                unsafe {
                    let this = self.as_mut().get_unchecked_mut();
                    mem::replace(this, CachedFuture::Avail(inner));
                }
                return Poll::Ready(());
            },
            CachedFutureProj::Avail(_inner) => return Poll::Ready(())
        }
    }
}

impl<F: Future> CachedFuture<F> {
    pub fn new(future: F) -> Self {
        Self::Fut(future)
    }

    pub fn ready(item: F::Output) -> Self {
        Self::Avail(item)
    }

    pub fn take_output(self) -> Option<F::Output> {
        match self {
            Self::Fut(_) => None,
            Self::Avail(output) => Some(output),
        }
    }

    pub fn replace(&mut self, future: F) -> Self {
        mem::replace(self, Self::new(future))
    }

    pub fn map<U>(self, f: impl FnOnce(F::Output) -> U) -> CachedFuture<impl Future<Output = U>> {
        match self {
            CachedFuture::Fut(fut) => {
                CachedFuture::Fut(async move {
                    f(fut.await)
                })
            },
            CachedFuture::Avail(output) => {
                CachedFuture::Avail(f(output))
            }
        }
    }

    pub fn then<U, UFut>(self, f: impl FnOnce(F::Output) -> UFut) -> CachedFuture<impl Future<Output = U>> 
    where 
        UFut: Future<Output = U>
    {
        CachedFuture::Fut(async move {
            match self {
                CachedFuture::Fut(fut) => {
                    f(fut.await).await
                },
                CachedFuture::Avail(output) => {
                    f(output).await
                }
            }
        })
    }
}

pub struct TakeCell<T> {
    item: Option<T>,
}

impl<T> TakeCell<T> {
    pub fn new(item: T) -> Self {
        Self { item: Some(item) }
    }

    pub fn take(&mut self) -> Option<TakeGuard<'_, T>> {
        if let Some(item) = self.item.take() {
            return Some(TakeGuard { return_address: &mut self.item, item: Some(item) })
        } else {
            return None
        }
    }
}

pub struct TakeGuard<'a, T> {
    return_address: &'a mut Option<T>,
    item: Option<T>
}

impl <'a, T> TakeGuard<'a, T> {
    pub fn get(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }

    pub fn get_ref(&self) -> &T {
        &self.item.as_ref().unwrap()
    }

    pub fn into_inner(mut self) -> T {
        self.item.take().unwrap()
    }
}

impl<'a, T> Drop for TakeGuard<'a, T> {
    fn drop(&mut self) {
        let item = self.item.take().unwrap();
        *self.return_address = Some(item)
    }
}

impl<'a, T> std::ops::Deref for TakeGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item.as_ref().unwrap()
    }
}

impl<'a, T> std::ops::DerefMut for TakeGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item.as_mut().unwrap()
    }
}

pin_project! {
    pub struct ErrorHandler<Inner, InnerError, E, Fut> 
    where
        Fut: Future<Output = Result<(), E>>,
    {
        inner: Pin<Box<Inner>>,
        fut: Option<Pin<Box<Fut>>>,

        handler: Box<dyn FnMut(&mut Pin<Box<Inner>>, InnerError) -> Fut>,
    }
}

impl<Inner, InnerError, E, Fut> ErrorHandler<Inner, InnerError, E, Fut> 
where
        Fut: Future<Output = Result<(), E>>,
{    

    pub fn new(inner: Inner, handler: impl FnMut(&mut Pin<Box<Inner>>, InnerError) -> Fut + 'static) -> Self {
        Self { inner: Box::pin(inner), fut: None, handler: Box::new(handler) }
    }


    fn poll<T>(
        mut self: Pin<&mut Self>,        
        mut poll_inner: impl FnMut(
            Pin<&mut Inner>,
            &mut std::task::Context<'_>,
        ) -> Poll<Result<T, InnerError>>, 
        cx: &mut std::task::Context<'_>
    ) -> Poll<Result<T, E>> {
        let this = self.as_mut().project();

        loop {
            match this.fut {
                Some(fut) => {
                    let pinned_fut = std::pin::pin!(fut);
                    match ready!(pinned_fut.poll(cx)) {
                        Ok(_inner) => {},
                        Err(e) => return Poll::Ready(Err(e))
                    }
                }
                None => {},
            };

            match ready!(poll_inner(this.inner.as_mut(), cx)) {
                Ok(item) => return Poll::Ready(Ok(item)),
                Err(inner_err) => {
                    let fut = (this.handler)(this.inner, inner_err);
                    *this.fut = Some(Box::pin(fut));
                    continue
                }
            }
        }
    }
}


impl<Inner, E, InnerItem, InnerError, Fut> Stream for ErrorHandler<Inner, InnerError, E, Fut>
where
    Fut: Future<Output = Result<(), E>>,
    Inner: Stream<Item = Result<InnerItem, InnerError>>,
{
    type Item = Result<InnerItem, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.poll(|inner, cx| {
            inner.poll_next(cx)
                .map(|opt| opt.transpose())
            }, cx)
            .map(|res| res.transpose())
    }
}

pub struct SendError;

impl<Inner, InnerError, E, InnerItem, Fut> Sink<InnerItem> for ErrorHandler<Inner, InnerError, E, Fut>
where
    Fut: Future<Output = Result<(), E>>,
    Inner: Sink<InnerItem, Error = InnerError>,
{
    type Error = Either<E, SendError>;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll(|inner, cx| inner.poll_ready(cx), cx)
            .map_err(Either::Left)

    }

    fn start_send(self: Pin<&mut Self>, item: InnerItem) -> Result<(), Self::Error> {
        let this = self.project();
        match this.inner.as_mut().start_send(item) {
            Ok(()) => return Ok(()),
            Err(_inner_err) => return Err(Either::Right(SendError))
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll(|inner, cx| inner.poll_flush(cx), cx)
            .map_err(Either::Left)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll(|inner, cx| inner.poll_close(cx), cx)
            .map_err(Either::Left)
    }
}

pub enum CloseEvent<Event> {
    Internal(Event),
    External(Event),
}

pub struct SinkClosedError;

pub trait ClosableSession<DataType, Event> {
    type Error;

    async fn close(&mut self, event: Event) -> Result<(), Self::Error>;
}

pub trait HasSession {
    type Session;
    fn session_mut(&mut self) -> &mut Self::Session;
}

impl<T, DataType, Event> ClosableSession<DataType, Event> for T
where 
    T: HasSession,
    T::Session: ClosableSession<DataType, Event>,
{
    type Error = <T::Session as ClosableSession<DataType, Event>>::Error;

    async fn close(&mut self, event: Event) -> Result<(), Self::Error> {
        self.session_mut().close(event).await
    }
}

pin_project! {
    pub struct SessionHandler<Inner, Event> 
    {
        state: Option<Either<Pin<Box<Inner>>, Pin<Box<dyn Future<Output = Option<Pin<Box<Inner>>>>>>>>,

        handler: Box<dyn FnMut(Pin<Box<Inner>>, Option<CloseEvent<Event>>) -> Pin<Box<dyn Future<Output = Option<Pin<Box<Inner>>>>>>>,
    }
}

impl<Inner, Event> SessionHandler<Inner, Event> 
{
    pub fn new<F, Fut>(inner: Inner, mut handler: F) -> Self 
    where 
        F: FnMut(Pin<Box<Inner>>, Option<CloseEvent<Event>>) -> Fut + 'static,
        Fut: Future<Output = Option<Pin<Box<Inner>>>> + 'static,
    {
        Self {
            state: Some(Either::Left(Box::pin(inner))),
            handler: Box::new(move |inner, event| {
                Box::pin(handler(inner, event))
            }),
        }
    }
}

impl<Inner, DataType, Event> ClosableSession<DataType, Event> for SessionHandler<Inner, Event> 
where 
    Inner: Sink<Either<DataType, Event>>,
    Self: Sink<DataType, Error = Either<SinkClosedError, Inner::Error>>,
{
    type Error = <Self as Sink<DataType>>::Error;
    
    async fn close(&mut self, event: Event) -> Result<(), Self::Error> {
        let item = Either::Right(event);

        match &mut self.state {
            Some(Either::Left(inner)) => {
                inner.as_mut().send(item).await.map_err(Either::Right)?;
                inner.as_mut().flush().await.map_err(Either::Right)?;
            }
            Some(Either::Right(fut)) => {
                if let Some(mut inner) = fut.await {
                    inner.as_mut().send(item).await.map_err(Either::Right)?;
                    inner.as_mut().flush().await.map_err(Either::Right)?;
                    self.state = Some(Either::Left(inner));
                } else {
                    return Err(Either::Left(SinkClosedError))
                }
            }
            None => {
                return Err(Either::Left(SinkClosedError))
            }
        }

        <Self as SinkExt<DataType>>::close(self).await
    }
}

impl<Inner, Event> SessionHandler<Inner, Event> 
{
    fn poll_state(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Pin<Box<Inner>>>> {
        let this = self.project();

        loop {
            match this.state.take() {
                Some(Either::Left(inner)) => {
                    return Poll::Ready(Some(inner))
                },
                Some(Either::Right(mut fut)) => {
                    match fut.poll_unpin(cx) {
                        Poll::Pending => {
                            *this.state = Some(Either::Right(fut));
                            return Poll::Pending
                        },
                        Poll::Ready(Some(new_inner)) => return Poll::Ready(Some(new_inner)),
                        Poll::Ready(None) => return Poll::Ready(None),
                    }
                },
                None => return Poll::Ready(None),
            } 
        }
    }
}

impl<Inner, Event, DataType> Stream for SessionHandler<Inner, Event>
where 
    Inner: Stream<Item = Either<DataType, Event>>,
{
    type Item = DataType;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match ready!(self.as_mut().poll_state(cx)) {
                Some(mut inner) => {
                    match inner.as_mut().poll_next(cx) {
                        Poll::Ready(Some(inner_item)) => {
                            match inner_item {
                                Either::Left(data) => {
                                    *self.as_mut().project().state = Some(Either::Left(inner));
                                    return Poll::Ready(Some(data))
                                },
                                Either::Right(event) => {
                                    let fut = (self.as_mut().handler)(inner, Some(CloseEvent::External(event)));
                                    *self.as_mut().project().state = Some(Either::Right(fut));
                                    continue
                                }

                            }
                        }
                        Poll::Ready(None) => {
                            let fut = (self.as_mut().handler)(inner, None);
                            *self.as_mut().project().state = Some(Either::Right(fut));
                            continue
                        },
                        Poll::Pending => {
                            *self.as_mut().project().state = Some(Either::Left(inner));
                            return Poll::Pending
                        }
                    }
                },
                None => return Poll::Ready(None),
            }
        }
    }
}

impl<Inner, Event, DataType> Sink<DataType> for SessionHandler<Inner, Event>
where 
    Inner: Sink<Either<DataType, Event>>,
{
    type Error = Either<SinkClosedError, Inner::Error>;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        if let Some(mut inner) = ready!(self.as_mut().poll_state(cx)) {
            let poll = inner.as_mut().poll_ready(cx);
            *self.project().state = Some(Either::Left(inner));
            return poll.map_err(Either::Right)
        }
        return Poll::Ready(Err(Either::Left(SinkClosedError)))
    }

    fn start_send(mut self: Pin<&mut Self>, item: DataType) -> Result<(), Self::Error> {
        let inner = match &mut self.state {
            Some(Either::Left(inner)) => inner,
            _ => panic!("start_send called before poll_ready"),
        };
        inner.as_mut().start_send(Either::Left(item)).map_err(Either::Right)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut inner = match ready!(self.as_mut().poll_state(cx)) {
            Some(inner) => inner,
            None => return Poll::Ready(Err(Either::Left(SinkClosedError)))
        };

        match inner.as_mut().poll_flush(cx) {
            Poll::Ready(Ok(())) => {
                *self.project().state = Some(Either::Left(inner));
                return Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                *self.project().state = Some(Either::Left(inner));
                return Poll::Ready(Err(Either::Right(e)))
            }
            Poll::Pending => {
                *self.project().state = Some(Either::Left(inner));
                return Poll::Pending
            }
        }

    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;

        match ready!(self.as_mut().poll_state(cx)) {
            Some(mut inner) => {
                match inner.as_mut().poll_close(cx) {
                    Poll::Ready(Ok(())) => {
                        *self.project().state = Some(Either::Left(inner));
                        return Poll::Ready(Ok(()))
                    }
                    Poll::Ready(Err(e)) => {
                        *self.project().state = Some(Either::Left(inner));
                        return Poll::Ready(Err(Either::Right(e)))
                    }
                    Poll::Pending => {
                        *self.project().state = Some(Either::Left(inner));
                        return Poll::Pending
                    }
                }
            }
            None => {
                return Poll::Ready(Err(Either::Left(SinkClosedError)))
            }
        }
    }
}

#[pin_project::pin_project()]
pub struct WithState<Inner, S, F> {
    #[pin]
    inner: Inner,
    state: S,
    f: F,
}

impl<Inner, S, F> WithState<Inner, S, F> {
    pub fn wrap(inner: Inner, initial_state: S, f: F) -> Self {
        Self {
            inner,
            state: initial_state,
            f,
        }
    }
}

impl<Inner, S, F> Stream for WithState<Inner, S, F> 
where 
    Inner: Stream,
    F: Fn(&Inner::Item, &mut S) -> (),
{
    type Item = Inner::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if let Some(next) = ready!(this.inner.poll_next(cx)) {
            (this.f)(&next, this.state);
            return Poll::Ready(Some(next))
        } else {
            return Poll::Ready(None)
        };
    }
}

impl<Inner, S, F, Item> Sink<Item> for WithState<Inner, S, F> 
where 
    Inner: Sink<Item>,
    F: Fn(&Item, &mut S) -> (),
{
    type Error = Inner::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        let this = self.project();

        (this.f)(&item, this.state);
        this.inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}







