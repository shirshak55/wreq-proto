use bytes::Bytes;
use http::HeaderValue;

/// The header fields of a received HTTP/1 response in wire order, each name spelled as
/// the peer sent it — a [`http::HeaderMap`] lowercases names and groups repeats.
#[derive(Clone, Debug, Default)]
pub struct RawHeaders(pub(crate) Vec<(Bytes, HeaderValue)>);

impl RawHeaders {
    /// Each field's name as sent and its value, in the order received.
    pub fn iter(&self) -> impl Iterator<Item = (&[u8], &HeaderValue)> {
        self.0.iter().map(|(name, value)| (name.as_ref(), value))
    }
}
