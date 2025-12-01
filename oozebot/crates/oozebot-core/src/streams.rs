
use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap, HashSet}, fmt::Debug, marker::PhantomData, pin::{pin, Pin}, task::Poll};

use futures::{future::{Either, Select}, stream::{self, Peekable, PollNext}, FutureExt, Stream, StreamExt};
use pin_project_lite::pin_project;
use tokio::{sync::mpsc::{channel, Receiver}, time::{sleep, sleep_until, Sleep}, time::{Duration, Instant}};
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



#[cfg(test)]
mod tests {
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
}


