/// Creates and polls a future on an interval, producing a stream.
#[cfg(feature = "interval_stream")]
mod interval_stream;
/// Allows attaching names to [`tokio::task`] to enable tracking which tasks are
/// exiting.
#[cfg(feature = "named_task")]
mod named_task;

#[cfg(feature = "interval_stream")]
pub use interval_stream::*;
#[cfg(feature = "named_task")]
pub use named_task::*;
