//! Extensions for HTTP messages in wreq_proto.

mod expect_continue;
mod h1_reason_phrase;
mod host_as_authority;
mod informational;
mod no_implied_content_length;
mod preserve_header;
mod raw_chunks;
mod raw_headers;
mod raw_request;
mod server_push;

pub use self::{
    expect_continue::ExpectContinue,
    h1_reason_phrase::ReasonPhrase,
    host_as_authority::HostAsAuthority,
    informational::on_informational,
    no_implied_content_length::NoImpliedContentLength,
    preserve_header::{on_preserve_header, OnPreserveHeaderCallback},
    raw_chunks::{record_response_chunks, RawChunks},
    raw_headers::{RawHeaders, RawTrailers},
    raw_request::{FieldSpacing, RawRequestTarget},
    server_push::{PushedResponse, ServerPush},
};
pub(crate) use self::{
    informational::OnInformational, preserve_header::OnPreserveHeader,
    raw_chunks::RecordResponseChunks,
};
