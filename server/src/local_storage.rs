use std::fs;
use rootly_lib::storage_api::Storage;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct LocalStorage {
    path: PathBuf,
}

impl LocalStorage {
    pub fn new(p: &str) -> LocalStorage {
        LocalStorage { path: PathBuf::from(p) }
    }
}

impl Storage for LocalStorage {
    fn read_config(&self) -> Option<Vec<u8>> {
        // Implement reading config from local storage
        let ini_file_path = self.path.join("ini.ry");
        let mut file = OpenOptions::new().read(true).open(&ini_file_path).ok();
        if let Some(ref mut f) = file {
            println!("Reading config from {}", ini_file_path.display());
            let mut buffer = fs::read(&ini_file_path).ok()?;
            println!("Read config from {} Done!", ini_file_path.display());
            Some(buffer)
        } else {
            None
        }
    }

    fn write_config(&self, config: Vec<u8>) {
        // Implement writing config to local storage
        let ini_file_path = self.path.join("ini.ry");
        let mut file = OpenOptions::new().write(true).create(true).open(&ini_file_path).unwrap();
        let status = file.write_all(&config);
        println!("Wrote config to {:?} Done!", status);
    }
}
