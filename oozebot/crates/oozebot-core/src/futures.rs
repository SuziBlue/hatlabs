use crate::connection::ConnectionError;





struct ConnectionFuture;

impl Future for ConnectionFuture {
    type Output = Result<(),ConnectionError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!();
    }
}
