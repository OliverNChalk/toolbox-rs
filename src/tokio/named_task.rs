use std::future::Future;
use std::pin::Pin;

use tokio::task::JoinError;

/// A wrapper around a [tokio::task::JoinHandle] that attaches a name.
///
/// This enables a parent task to await all children simultaneously but still
/// determine what child has exited for logging/diagnosis purposes.
#[derive(Debug)]
pub struct NamedTask<Ret = (), Name = String>
where
    Name: Clone + Unpin,
{
    task: Pin<Box<tokio::task::JoinHandle<Ret>>>,
    name: Name,
}

impl<Ret, Name> NamedTask<Ret, Name>
where
    Name: Clone + Unpin,
{
    pub fn new(task: tokio::task::JoinHandle<Ret>, name: Name) -> Self {
        NamedTask { task: Box::pin(task), name }
    }

    pub fn name(&self) -> &Name {
        &self.name
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
        self.task.as_mut().poll(cx).map(|v| (self.name.clone(), v))
    }
}
