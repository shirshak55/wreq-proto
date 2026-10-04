use bytes::Bytes;
use http::{uri::PathAndQuery, HeaderMap, HeaderName};

/// What a request's header field lines had between their names and values (the colon and
/// the whitespace around it) and after their values, as its client wrote them. An HTTP/1
/// request head written in a preserved order (see
/// [`on_preserve_header`](super::on_preserve_header)) writes each field line with what its
/// name's line in that place recorded, else with `: `.
#[derive(Clone, Debug, Default)]
pub struct FieldSpacing(HeaderMap<(Bytes, Bytes)>);

impl FieldSpacing {
    /// Records the next `name` field line's `separator`, its colon included, and the
    /// `trailing` whitespace after its value.
    pub fn append(&mut self, name: HeaderName, separator: Bytes, trailing: Bytes) {
        self.0.append(name, (separator, trailing));
    }

    /// What the `nth` `name` field line recorded.
    pub(crate) fn get(&self, name: &HeaderName, nth: usize) -> Option<&(Bytes, Bytes)> {
        self.0.get_all(name).iter().nth(nth)
    }
}

/// A request-target's path and query as its client sent them (`raw`, non-ASCII bytes not
/// percent-encoded), and as its `Uri` carries them (`encoded`). An HTTP/1 request head
/// writes `raw` while the request's `Uri` still carries `encoded`.
#[derive(Clone, Debug)]
pub struct RawRequestTarget {
    /// The path and query as sent.
    pub raw: Bytes,
    /// The path and query as the request's `Uri` carries them.
    pub encoded: PathAndQuery,
}
