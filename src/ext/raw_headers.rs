use std::sync::{Arc, OnceLock};

use bytes::Bytes;
use http::HeaderValue;

/// The header fields of a received response in wire order, each name spelled as the peer
/// sent it (lowercase over HTTP/2) — a [`http::HeaderMap`] lowercases names and groups
/// repeats.
#[derive(Clone, Debug, Default)]
pub struct RawHeaders(pub(crate) Vec<(Bytes, HeaderValue)>);

impl RawHeaders {
    /// Each field's name as sent and its value, in the order received.
    pub fn iter(&self) -> impl Iterator<Item = (&[u8], &HeaderValue)> {
        self.0.iter().map(|(name, value)| (name.as_ref(), value))
    }
}

/// A message's trailer fields in wire order, each name spelled as sent (lowercase over
/// HTTP/2), which the trailers `HeaderMap` its body yields lowercases and groups.
///
/// A received HTTP/1 chunked or HTTP/2 response carries one, filled as its trailers are
/// read, before the body yields them. A request sent carrying a filled one writes its
/// trailers in that spelling and order. Clones share the fields, so a proxy can hand a
/// received message's record (or its cell, from another HTTP library) to the request it
/// relays before the trailers arrive.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawTrailers(pub Arc<OnceLock<Vec<(Bytes, HeaderValue)>>>);

/// What each trailer field of a message had between its name and its value (the colon
/// included), and after its value up to its line ending, in the order of its
/// [`RawTrailers`]. A request sent over HTTP/1 carrying a filled one with its `RawTrailers`
/// writes its trailers so. Clones share the record, like `RawTrailers`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TrailerSpacing(pub Arc<OnceLock<Vec<(Bytes, Bytes)>>>);
