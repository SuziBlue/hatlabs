use std::sync::mpsc;


#[derive(Debug, Clone)]
pub struct StdSender<T>(pub mpsc::Sender<T>);

impl<T: Send + Sync + Clone> ui_core::comms::Sender<T> for StdSender<T> {
    fn try_send(&self, msg: T) -> Result<(), ui_core::comms::SendError> {
        self.0.send(msg).map_err(|_| ui_core::comms::SendError::Disconnected)
    }
}

#[derive(Debug)]
pub struct StdReceiver<T>(pub mpsc::Receiver<T>);

impl<T: Send + Sync + Clone> ui_core::comms::Receiver<T> for StdReceiver<T> {
    fn try_recv(&mut self) -> Result<T, ui_core::comms::RecvError> {
        self.0.try_recv().map_err(|err| match err {
            mpsc::TryRecvError::Empty => ui_core::comms::RecvError::Empty,
            mpsc::TryRecvError::Disconnected => ui_core::comms::RecvError::Disconnected,
        })
    }
}
