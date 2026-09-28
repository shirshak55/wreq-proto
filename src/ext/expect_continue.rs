use std::{fmt, sync::Arc, time::Duration};

use crate::rt::Timer;

/// Makes the connection sending the request carrying it honour its
/// `Expect: 100-continue`: the head goes out, then the body is held back until the server
/// answers `100 Continue` (passed to [`on_informational`](super::on_informational) like
/// any interim response) or `timeout` passes. A final response arriving first means the
/// body is never sent; an HTTP/1 connection then closes after that response, and an
/// HTTP/2 stream is reset with `CANCEL` once the response is dropped.
///
/// Without this extension, or on a request without `Expect: 100-continue`, an empty body,
/// or HTTP/1.0, the body follows the head at once.
#[derive(Clone)]
pub struct ExpectContinue {
    pub(crate) timer: Arc<dyn Timer + Send + Sync>,
    pub(crate) timeout: Duration,
}

impl ExpectContinue {
    /// Waits for `100 Continue` up to `timeout`, measured by `timer` from when the head is
    /// sent (curl waits 1 second).
    pub fn new<T>(timer: T, timeout: Duration) -> Self
    where
        T: Timer + Send + Sync + 'static,
    {
        Self {
            timer: Arc::new(timer),
            timeout,
        }
    }

    /// Whether `headers` carry `Expect: 100-continue`.
    pub(crate) fn is_expected(headers: &http::HeaderMap) -> bool {
        headers
            .get(http::header::EXPECT)
            .is_some_and(|v| v.as_bytes().eq_ignore_ascii_case(b"100-continue"))
    }
}

impl fmt::Debug for ExpectContinue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpectContinue")
            .field("timeout", &self.timeout)
            .finish()
    }
}
