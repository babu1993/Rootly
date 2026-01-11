pub mod header;
mod mutable_headers;
pub mod log;
pub mod trace;
pub mod ini;

use std::mem::size_of;

pub const HEADER_SEGMENT_LENGTH: usize = 11;
pub const USIZE_SEGMENT_LENGTH: usize = size_of::<usize>();

pub trait ReadableModel {
    fn from_bytes(data: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn size_in_bytes(&self) -> usize;
}