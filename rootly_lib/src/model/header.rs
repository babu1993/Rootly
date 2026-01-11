use crate::model::ReadableModel;
use byteorder::{BigEndian, ByteOrder};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Header {
    version_major: u8,
    version_minor: u8,
    build_number: u8,
    timestamp: u64,
}

impl Header {
    pub fn new(version_major: u8, version_minor: u8, build_number: u8, timestamp: u64) -> Self {
        Header {
            version_major,
            version_minor,
            build_number,
            timestamp,
        }
    }

    pub fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Header {
            version_major: 1,
            version_minor: 0,
            build_number: 0,
            timestamp: now,
        }
    }
}

impl ReadableModel for Header {
    fn from_bytes(data: &[u8]) -> Self {
        let version_major = data[0];
        let version_minor = data[1];
        let build_number = data[2];
        let timestamp = BigEndian::read_u64(&data[3..11]);

        Header {
            version_major,
            version_minor,
            build_number,
            timestamp
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(super::HEADER_SEGMENT_LENGTH);
        bytes.push(self.version_major);
        bytes.push(self.version_minor);
        bytes.push(self.build_number);
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes
    }
    
    fn size_in_bytes(&self) -> usize {
        super::HEADER_SEGMENT_LENGTH
    }

}