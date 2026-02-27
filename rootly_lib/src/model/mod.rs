mod header;
mod mutable_headers;
mod log;
mod trace;
mod ini;

use std::mem::size_of;

pub const HEADER_SEGMENT_LENGTH: usize = 11;
pub const USIZE_SEGMENT_LENGTH: usize = size_of::<usize>();

pub trait ReadableModel {
    fn from_bytes(data: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn size_in_bytes(&self) -> usize;
}

pub mod prelude {
    pub use super::header::Header;
    pub use super::mutable_headers::MutableHeader;
    pub use super::log::{LogFile, Log, LogEntry};
    pub use super::trace::{Trace};
    pub use super::ini::IniFile;
}