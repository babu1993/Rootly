use byteorder::{BigEndian, ByteOrder};
use crate::model::ReadableModel;
use super::header::Header;
pub struct MutableHeader {
    header: Header,
    total_length: usize,
}

impl MutableHeader {
    pub fn new(header: Header, total_length: usize) -> Self {
        MutableHeader { header, total_length }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = header;
    }

    pub fn get_total_length(&self) -> usize {
        self.total_length
    }

    pub fn set_total_length(&mut self, length: usize) {
        self.total_length = length;
    }
}

impl ReadableModel for MutableHeader {
    fn from_bytes(data: &[u8]) -> Self {
        let header = Header::from_bytes(&data[0..super::HEADER_SEGMENT_LENGTH]);
        let total_length = BigEndian::read_u64(&data[super::HEADER_SEGMENT_LENGTH..]) as usize;
        MutableHeader { header, total_length}
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes();
        bytes.extend_from_slice(&self.total_length.to_be_bytes());
        bytes
    }

    fn size_in_bytes(&self) -> usize {
        Header::size_in_bytes(&self.header) + 8 // 8 bytes for usize
    }
}
