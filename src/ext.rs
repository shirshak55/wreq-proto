//! Extensions for HTTP messages in wreq_proto.

mod expect_continue;
mod h1_reason_phrase;
mod host_as_authority;
mod informational;
mod preserve_header;
mod raw_chunks;
mod raw_headers;

pub use self::{
    expect_continue::ExpectContinue,
    h1_reason_phrase::ReasonPhrase,
    host_as_authority::HostAsAuthority,
    informational::on_informational,
    preserve_header::{on_preserve_header, OnPreserveHeaderCallback},
    raw_chunks::{record_response_chunks, RawChunks},
    raw_headers::{RawHeaders, RawTrailers},
};
pub(crate) use self::{
    informational::OnInformational, preserve_header::OnPreserveHeader,
    raw_chunks::RecordResponseChunks,
};
