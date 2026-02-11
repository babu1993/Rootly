use std::path::Path;
use std::io::{Seek, SeekFrom};

pub const MUTABLE_LOGS_FILE_DIRECTORY: &str = "mt_logs";
pub const MUTABLE_TRACES_FILE_DIRECTORY: &str = "mt_traces";

pub trait Storage: Send + Sync {
    fn read_config(&self) -> Option<Vec<u8>>;
    fn write_config(&self, config: Vec<u8>);

    fn write(&self, file_name: &str, data: Vec<u8>, offset: Option<usize>);

    fn read(&self, file_name: &str, offset: Option<usize>) -> Vec<u8>;

}