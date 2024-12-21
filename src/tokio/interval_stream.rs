use std::future::Future;
use std::pin::pin;
use std::task::Poll;

use futures::{ready, Stream};

/// Creates and polls a future on an interval, producing a stream.
///
/// # Example
///
/// ```rust
/// # tokio_test::block_on(async {
/// use futures::FutureExt;
/// use futures::StreamExt;
/// use toolbox::tokio::IntervalStream;
///
/// let slot_interval = tokio::time::interval(std::time::Duration::from_millis(1));
/// let mut slot_stream = IntervalStream::new(
///     slot_interval,
///     Box::new(|| futures::future::ready("hello world").boxed()),
/// );
///
/// assert_eq!(slot_stream.next().await.unwrap(), "hello world");
/// assert_eq!(slot_stream.next().await.unwrap(), "hello world");
/// # })
/// ```
pub struct IntervalStream<Fut>
where
    Fut: Unpin,
{
    interval: tokio::time::Interval,
    poll: Box<dyn Fn() -> Fut>,

    in_progress: Option<Fut>,
}

impl<Fut, Output> IntervalStream<Fut>
where
    Fut: Future<Output = Output> + Unpin,
{
    pub fn new(interval: tokio::time::Interval, poll: Box<dyn Fn() -> Fut>) -> Self {
        IntervalStream { interval, poll, in_progress: None }
    }
}

impl<Fut, Output> Stream for IntervalStream<Fut>
where
    Fut: Future<Output = Output> + Unpin,
{
    type Item = Output;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // Poll the current future if one already exists.
        if let Some(fut) = &mut this.in_progress {
            let output = ready!(pin!(fut).poll(cx));
            this.in_progress = None;

            return Poll::Ready(Some(output));
        }

        // Poll the interval to see if we should create a new future.
        ready!(this.interval.poll_tick(cx));

        // Create a new future.
        let mut fut = (this.poll)();

        // Poll the future.
        let pinned = pin!(&mut fut);
        let poll = pinned.poll(cx);
        match poll {
            Poll::Ready(output) => Poll::Ready(Some(output)),
            Poll::Pending => {
                this.in_progress = Some(fut);

                Poll::Pending
            }
        }
    }
}
