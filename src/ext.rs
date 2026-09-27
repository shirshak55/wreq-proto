//! Extensions for HTTP messages in wreq_proto.

mod h1_reason_phrase;
mod informational;
mod preserve_header;
mod raw_headers;

pub use self::{
    h1_reason_phrase::ReasonPhrase,
    raw_headers::RawHeaders,
    informational::on_informational,
    preserve_header::{on_preserve_header, OnPreserveHeaderCallback},
};
pub(crate) use self::{informational::OnInformational, preserve_header::OnPreserveHeader};
