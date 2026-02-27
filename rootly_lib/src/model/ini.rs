use crate::model::header::Header;
use crate::model::ReadableModel;
use std::time::SystemTime;

#[derive(Debug)]
pub struct IniFile {
    header: Header,
    mutable_logs_file:String,
    mutable_trace_file:String,
}

impl IniFile {
    pub fn build_default() -> Self {
        let current_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        IniFile {
            header: Header::default(),
            mutable_logs_file: format!("logs_mutable_{}.dat", current_epoch.as_millis()),
            mutable_trace_file: format!("traces_mutable_{}.dat", current_epoch.as_millis()),
        }
    }

    pub fn get_logs_file_name(&self) -> &String {
        &self.mutable_logs_file
    }

    pub fn get_trace_file_name(&self) -> &String {
        &self.mutable_trace_file
    }

    pub fn save(&self, storage: &Box<dyn crate::storage_api::Storage>) {
        storage.write("rootly", self.to_bytes(), None);
    }
}

impl ReadableModel for IniFile {
    fn from_bytes(data: &[u8]) -> Self {
        let mut offset;
        let header = Header::from_bytes(&data[0..super::HEADER_SEGMENT_LENGTH]);
        offset = super::HEADER_SEGMENT_LENGTH;
        // Further implementation to read mutable file names from bytes
        let mutable_logs_file_len = usize::from_be_bytes(data[offset..offset+super::USIZE_SEGMENT_LENGTH].try_into().unwrap());
        offset += super::USIZE_SEGMENT_LENGTH;
        let mutable_logs_file = String::from_utf8(data[offset..offset+mutable_logs_file_len].to_vec()).unwrap();
        offset += mutable_logs_file_len;
        let mutable_trace_file_len = usize::from_be_bytes(data[offset..offset+super::USIZE_SEGMENT_LENGTH].try_into().unwrap());
        offset += super::USIZE_SEGMENT_LENGTH;
        let mutable_trace_file = String::from_utf8(data[offset..offset+mutable_trace_file_len].to_vec()).unwrap();
        IniFile {
            header,
            mutable_logs_file,
            mutable_trace_file,
        }

    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size_in_bytes());
        bytes.extend_from_slice(&self.header.to_bytes());
        let mutable_logs_file_bytes = self.mutable_logs_file.as_bytes();
        let mutable_logs_file_len = mutable_logs_file_bytes.len();
        bytes.extend_from_slice(&mutable_logs_file_len.to_be_bytes());
        bytes.extend_from_slice(mutable_logs_file_bytes);
        let mutable_trace_file_bytes = self.mutable_trace_file.as_bytes();
        let mutable_trace_file_len = mutable_trace_file_bytes.len();
        bytes.extend_from_slice(&mutable_trace_file_len.to_be_bytes());
        bytes.extend_from_slice(mutable_trace_file_bytes);
        bytes
    }

    fn size_in_bytes(&self) -> usize {
        // Implementation to get size in bytes
        super::HEADER_SEGMENT_LENGTH + 2 + self.mutable_logs_file.len() + 2 + self.mutable_trace_file.len()
    }
}