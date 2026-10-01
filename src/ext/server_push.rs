use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::mpsc::UnboundedSender;

use crate::{body::Incoming, Result};

/// Relays the pushes an HTTP/2 server promises on the request carrying it (RFC 9113 §8.4):
/// each promised request's head and its [`PushedResponse`] are sent as the promise arrives,
/// until no more can come on the request's stream (its response ended, or it was reset) or
/// the receiver is dropped. Without it, a promised push is cancelled once the response is
/// dropped. Ignored over HTTP/1.
#[derive(Clone, Debug)]
pub struct ServerPush(pub UnboundedSender<(http::Request<()>, PushedResponse)>);

/// The response an HTTP/2 server pushes for a promised request, as its head arrives. Dropped
/// before its end, as is its body, it cancels the push.
pub struct PushedResponse(
    pub(crate) Pin<Box<dyn Future<Output = Result<http::Response<Incoming>>> + Send>>,
);

impl Future for PushedResponse {
    type Output = Result<http::Response<Incoming>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

impl fmt::Debug for PushedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PushedResponse").finish_non_exhaustive()
    }
}
