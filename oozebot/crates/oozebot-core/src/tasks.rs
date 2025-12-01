use std::future;

use futures::{future::{FusedFuture, Then}, stream::{AbortHandle, AbortRegistration, Abortable, Aborted}, FutureExt};
use pin_project_lite::pin_project;





pin_project! {
    #[derive(Debug)]
    #[project = ConnectionTaskProj]
    pub enum ConnectionTask<Fut, C, CFut>
    {
        Incomplete {
            #[pin]
            inner: Then<Abortable<Fut>, CFut, C>,
        },
        Complete,
    }
}

impl<Fut, C, CFut, T> ConnectionTask<Fut, C, CFut> 
where 
    Fut: Future<Output = T>,
    C: FnOnce(Result<T, Aborted>) -> CFut,
    CFut: Future<Output = Result<T, Aborted>>,
{
    pub fn new<F>(task: F, cleanup: C, abort_registration: AbortRegistration) -> Self 
    where 
        F: FnOnce() -> Fut,
    {
        let inner = Abortable::new(task(), abort_registration)
            .then(cleanup);
        Self::Incomplete{ inner }
    }
}



impl<Fut, C, CFut, T> Future for ConnectionTask<Fut, C, CFut> 
where 
    Fut: Future<Output = T>,
    C: FnOnce(Result<T, Aborted>) -> CFut,
    CFut: Future<Output = Result<T, Aborted>>,
{
    type Output = Result<T, Aborted>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.project() {
            ConnectionTaskProj::Incomplete { inner } => inner.poll(cx),
            ConnectionTaskProj::Complete => panic!("Connection must not be polled after it returned 'Poll::Ready'")
        }
    }
}

impl<Fut, C, CFut, T> FusedFuture for ConnectionTask<Fut, C, CFut> 
where 
    Fut: Future<Output = T>,
    C: FnOnce(Result<T, Aborted>) -> CFut,
    CFut: Future<Output = Result<T, Aborted>>, 
{
    fn is_terminated(&self) -> bool {
        match self {
            Self::Incomplete { .. } => false,
            Self::Complete => true,
        }
    }
}
