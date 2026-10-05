use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use bytes::Bytes;

/// A chunked HTTP/1 body's chunk-size lines as received, in order: each chunk's size and
/// its line as sent (the size digits' spelling, any whitespace, and the chunk extensions,
/// without the CRLF), the last chunk (size 0) included.
///
/// The chunked response to a request marked with [`record_response_chunks`] carries one,
/// filled with each line as it is read, before the chunk's data. A chunked HTTP/1 request
/// sent carrying one splits its body at the recorded sizes and writes each recorded line,
/// whatever frames the body yields, so a proxy relaying a received body keeps its chunks
/// and their extensions; bytes past the record go out one chunk per frame, and a body
/// ending inside a recorded chunk is an error. It drops each line from the record once
/// written, so a long body's record holds only the lines still to go out (the last
/// chunk's stays). Clones share the lines, so a proxy can hand a received message's
/// record (or its cell, from another HTTP library) to the request it relays as the lines
/// arrive; one also reading them takes them from the received record as they arrive,
/// handing them on to a record of its own that the relayed request carries.
#[derive(Clone, Debug, Default)]
pub struct RawChunks(pub Arc<Mutex<Vec<(u64, Bytes)>>>);

impl RawChunks {
    pub(crate) fn lock(&self) -> MutexGuard<'_, Vec<(u64, Bytes)>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl PartialEq for RawChunks {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Marks a request whose chunked HTTP/1 response should carry a [`RawChunks`] record.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RecordResponseChunks;

/// Asks the HTTP/1 connection sending `req` to record the chunk-size lines of its response
/// in a [`RawChunks`] extension on the response, if it is chunked.
pub fn record_response_chunks<B>(req: &mut http::Request<B>) {
    req.extensions_mut().insert(RecordResponseChunks);
}
