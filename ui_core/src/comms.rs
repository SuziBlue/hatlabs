use core::fmt;
pub use leylines::{ChannelState, Receiver, Sender};

pub trait ThreadSpawner: Clone + Send + Sync + 'static {
    fn spawn<F>(&self, future: F) -> impl ThreadHandle
    where 
        F: core::future::Future<Output = ()> + Send + 'static;
}
pub trait ThreadHandle {
    fn join(self) -> Result<(), ActorError>;
}

pub trait Actor{
    fn start(&self, spawner: impl ThreadSpawner) -> impl ThreadHandle;
}

#[derive(Debug)]
pub enum ActorError {
    Channel(ChannelState),
    JoinFailed,
    // Add other variants as needed
}

impl fmt::Display for ActorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorError::Channel(state) => write!(f, "Channel error: {}", state),
            ActorError::JoinFailed => write!(f, "Thread join failed"),
        }
    }
}

impl core::error::Error for ActorError {}

impl From<ChannelState> for ActorError {
    fn from(state: ChannelState) -> Self {
        ActorError::Channel(state)
    }
}

pub trait Source<T>: Actor {
    fn connect(&self) -> impl Receiver<T>;
}

pub trait Sink<T>: Actor {
    fn connect(&self) -> impl Sender<T>;
}
//
//pub trait Sender<T>: Send + Sync + Clone {
//    fn try_send(&self, msg: T) -> Result<(), SendError>;
//}
//pub trait Receiver<T>: Send {
//    fn try_recv(&mut self) -> Result<T, RecvError>;
//}
//
//#[derive(Debug)]
//pub struct ActorError;
//
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//pub enum SendError {
//    Full,
//    Disconnected,
//}
//
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//pub enum RecvError {
//    Empty,
//    Disconnected,
//}
//
//impl fmt::Display for SendError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            SendError::Full => write!(f, "Channel is full"),
//            SendError::Disconnected => write!(f, "Channel is disconnected"),
//        }
//    }
//}
//
//impl fmt::Display for RecvError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            RecvError::Empty => write!(f, "Channel is empty"),
//            RecvError::Disconnected => write!(f, "Channel is disconnected"),
//        }
//    }
//}
//
//impl Error for SendError {}
//impl Error for RecvError {}
//
//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//pub enum ChannelError {
//    Send(SendError),
//    Recv(RecvError),
//}
//
//impl From<SendError> for ChannelError {
//    fn from(e: SendError) -> Self {
//        ChannelError::Send(e)
//    }
//}
//
//impl From<RecvError> for ChannelError {
//    fn from(e: RecvError) -> Self {
//        ChannelError::Recv(e)
//    }
//}
//
//impl fmt::Display for ChannelError {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        match self {
//            ChannelError::Send(e) => write!(f, "Send error: {}", e),
//            ChannelError::Recv(e) => write!(f, "Receive error: {}", e),
//        }
//    }
//}
//
//impl Error for ChannelError {
//    fn source(&self) -> Option<&(dyn Error + 'static)> {
//        match self {
//            ChannelError::Send(e) => Some(e),
//            ChannelError::Recv(e) => Some(e),
//        }
//    }
//}
