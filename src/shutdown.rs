use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio_util::sync::{CancellationToken, WaitForCancellationFuture};

#[derive(Debug, Clone)]
pub struct Shutdown {
    pub token: CancellationToken,
    pub shutdown: Arc<AtomicBool>,
}

impl Shutdown {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self { token: CancellationToken::new(), shutdown: Arc::new(AtomicBool::new(false)) }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.token.cancel();
    }

    #[must_use]
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Relaxed)
    }

    pub fn cancelled(&self) -> WaitForCancellationFuture<'_> {
        self.token.cancelled()
    }
}
