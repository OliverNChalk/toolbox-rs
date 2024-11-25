use std::future::Future;
use std::pin::Pin;

use tokio::task::JoinError;

/// A wrapper around a [tokio::task::JoinHandle] that attaches a name.
///
/// This enables a parent task to await all children simultaneously but still
/// determine what child has exited for logging/diagnosis purposes.
#[derive(Debug)]
pub struct NamedTask<Ret = (), Id = String>
where
    Id: Clone + Unpin,
{
    task: Pin<Box<tokio::task::JoinHandle<Ret>>>,
    id: Id,
}

impl<R, I> NamedTask<R, I>
where
    I: Clone + Unpin,
{
    pub fn new(task: tokio::task::JoinHandle<R>, id: I) -> Self {
        NamedTask { task: Box::pin(task), id }
    }
}

impl<R, I> Future for NamedTask<R, I>
where
    I: Clone + Unpin,
{
    type Output = (I, Result<R, JoinError>);

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.task.as_mut().poll(cx).map(|v| (self.id.clone(), v))
    }
}
