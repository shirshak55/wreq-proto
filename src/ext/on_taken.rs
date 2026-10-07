use std::{fmt, sync::Arc};

/// Called once the HTTP/1 connection the request carrying it goes over takes it to write:
/// one its sender dropped before is never written, while one taken is written though its
/// response is no longer awaited (its body as far as that goes). Ignored over HTTP/2.
#[derive(Clone)]
pub struct OnTaken(Arc<dyn Fn() + Send + Sync>);

impl OnTaken {
    /// Calls `taken` once the request is taken.
    pub fn new<F>(taken: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self(Arc::new(taken))
    }

    pub(crate) fn taken(&self) {
        (self.0)()
    }
}

impl fmt::Debug for OnTaken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OnTaken").finish_non_exhaustive()
    }
}
