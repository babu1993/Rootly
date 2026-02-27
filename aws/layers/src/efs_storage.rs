use rootly_lib::prelude::Storage;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub struct EfsStorage{
    mnt_path: String
}

impl EfsStorage{
    pub fn new() -> EfsStorage {
        EfsStorage { mnt_path: "/mnt/rootly".to_string() }
    }

    fn get_file_path(&self, file_name: &str) -> PathBuf {
        let file_path = Path::new(&self.mnt_path).join(file_name);
        file_path
    }
}
impl Storage for EfsStorage {
    fn write(&self, file_name: &str, data: Vec<u8>, offset: Option<usize>) {
        let file_path = self.get_file_path(file_name);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .expect("Unable to open file for writing");

        if let Some(offset) = offset {
            file.seek(SeekFrom::Start(offset as u64)).expect("Unable to seek in file");
        }

        file.write_all(&data).expect("Unable to write data to file");
    }

    fn read(&self, file_name: &str, offset: Option<usize>, size: Option<usize>) -> Option<Vec<u8>> {
        let file_path = self.get_file_path(file_name);
        let mut file = File::open(file_path).ok()?;

        if let Some(offset) = offset {
            file.seek(SeekFrom::Start(offset as u64)).ok()?;
        }
        let mut buffer: Vec<u8>;
        if size.is_some() {
            buffer = vec![0; size.unwrap()];
            let bytes_read = file.read(&mut buffer).ok()?;
        }
        else {
            buffer = Vec::new();
            file.read_to_end(&mut buffer).ok()?;
        }
        Some(buffer)
    }

    fn delete(&self, file_name: &str) {
        let file_path = self.get_file_path(file_name);
        fs::remove_file(file_path).expect("Unable to delete file");
    }

    fn is_file_exists(&self, file_name: &str) -> bool {
        let file_path = self.get_file_path(file_name);
        Path::new(&file_path).exists()
    }
}

