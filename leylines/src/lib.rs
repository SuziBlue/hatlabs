#![no_std]

use core::future::Future;
use core::fmt;

/// State of the channel for sending and receiving operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelState {
    Closed,
    Full,
    Empty,
}

impl fmt::Display for ChannelState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChannelState::Closed => write!(f, "Channel is closed"),
            ChannelState::Full => write!(f, "Channel is full"),
            ChannelState::Empty => write!(f, "Channel is empty"),
        }
    }
}

impl core::error::Error for ChannelState {}

/// Trait for sending values into a channel.
pub trait Sender<T>: Clone + Send + Sync {
    /// The associated future for `send_async`.
    type SendFuture<'a>: Future<Output = Result<(), ChannelState>> + Send
    where
        T: 'a,
        Self: 'a;

    /// Send a value, blocking until the value is sent or the channel is closed.
    fn send(&self, value: T) -> Result<(), ChannelState>;

    /// Try to send a value immediately.
    fn try_send(&self, value: T) -> Result<(), ChannelState>;

    /// Send asynchronously. Returns a future that resolves when the value is sent or the channel is closed.
    fn send_async(&self, value: T) -> Self::SendFuture<'_>;
}

pub trait BroadcastSender<T>: Sender<T> {
    /// Subscribe a new receiver to the broadcast stream.
    fn subscribe(&self) -> impl Receiver<T>;
}

/// Trait for receiving values from a channel.
pub trait Receiver<T>: Send + Sync {
    /// The associated future for `recv_async`.
    type RecvFuture<'a>: Future<Output = Result<T, ChannelState>> + Send
    where
        T: 'a,
        Self: 'a;

    /// Receive a value, blocking if necessary.
    fn recv(&self) -> Result<T, ChannelState>;

    /// Try to receive a value immediately.
    fn try_recv(&self) -> Result<T, ChannelState>;

    /// Receive asynchronously. Returns a future that resolves when a value is available or the channel is closed.
    fn recv_async(&self) -> Self::RecvFuture<'_>;
}

/// Trait for a basic sender/receiver channel abstraction.
pub trait Channel<S, R, T>
where
    S: Sender<T>,
    R: Receiver<T>,
{
    /// Open a new channel with the given capacity.
    fn open(size: usize) -> (S, R);
}

/// A broadcast channel where all receivers receive every message sent.
pub trait BroadcastChannel<S, R, T>: Channel<S, R, T>
where
    S: BroadcastSender<T>,
    R: Receiver<T>,
{}

/// A multi-consumer channel where receivers share a queue.
pub trait MultiConsumerChannel<S, R, T>: Channel<S, R, T>
where
    S: Sender<T>,
    R: Receiver<T> + Clone,
{}
