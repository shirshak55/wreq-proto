/// Sends an HTTP/2 request's `Host` field as its `:authority`, in place of its URI's
/// authority: a proxy relaying an HTTP/1 request, whose `Host` names the authority, or a
/// `Host` edited to retarget one. HTTP/2 carries that as `:authority` (RFC 9113 section
/// 8.3.1); without this extension a `Host` goes out as a field beside it, as the caller set
/// it.
///
/// `keep_field` also sends the `Host` field itself, where it stands; otherwise only the
/// `:authority` carries it. A value that isn't an authority stays a field. Ignored over
/// HTTP/1, whose `Host` is the authority.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HostAsAuthority {
    /// Whether the `Host` field is sent as well.
    pub keep_field: bool,
}
