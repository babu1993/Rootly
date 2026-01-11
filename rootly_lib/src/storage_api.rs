use std::path::Path;
use std::io::{Seek, SeekFrom};

pub trait Storage: Send + Sync {
    fn read_config(&self) -> Option<Vec<u8>>;
    fn write_config(&self, config: Vec<u8>);
}