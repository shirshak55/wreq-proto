/// Sends an HTTP/2 request without a `Content-Length` it doesn't carry: none is implied
/// from its body's exact size, as a proxy relays a request its client sent without one.
/// Ignored over HTTP/1, which frames a body by its length.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoImpliedContentLength;
