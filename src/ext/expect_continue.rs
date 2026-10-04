use std::{
    fmt,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

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
    pub(crate) wait: Option<(Arc<dyn Timer + Send + Sync>, Duration)>,
}

impl ExpectContinue {
    /// Waits for `100 Continue` up to `timeout`, measured by `timer` from when the head is
    /// sent (curl waits 1 second).
    pub fn new<T>(timer: T, timeout: Duration) -> Self
    where
        T: Timer + Send + Sync + 'static,
    {
        Self {
            wait: Some((Arc::new(timer), timeout)),
        }
    }

    /// Sends the body as soon as it yields data, for a body whose producer already waits
    /// for `100 Continue` (a proxy relaying its client's body). Over HTTP/1, a final
    /// response arriving before any of the body was written still closes the connection
    /// after it.
    pub fn relayed() -> Self {
        Self { wait: None }
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
            .field("timeout", &self.wait.as_ref().map(|(_, timeout)| timeout))
            .finish()
    }
}

/// Ends the sending side of the HTTP/1 connection a request carrying it goes over once
/// the request is written and `poll` is ready: a proxy relaying a client that half-closed
/// its connection (a TCP FIN) mid-request half-closes too.
#[derive(Clone)]
pub struct ReadClosed(Arc<dyn Fn(&mut Context<'_>) -> Poll<()> + Send + Sync>);

impl ReadClosed {
    /// Half-closes once `poll` is ready, which registers the waker it is given otherwise.
    pub fn new<F>(poll: F) -> Self
    where
        F: Fn(&mut Context<'_>) -> Poll<()> + Send + Sync + 'static,
    {
        Self(Arc::new(poll))
    }

    pub(crate) fn poll_closed(&self, cx: &mut Context<'_>) -> Poll<()> {
        (self.0)(cx)
    }
}

impl fmt::Debug for ReadClosed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadClosed").finish_non_exhaustive()
    }
}
