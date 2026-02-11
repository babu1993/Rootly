use rootly_lib::storage_api::Storage;
use rootly_lib::storage_api::{MUTABLE_LOGS_FILE_DIRECTORY, MUTABLE_TRACES_FILE_DIRECTORY};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
#[derive(Debug)]
pub struct LocalStorage {
    path: PathBuf,
}

impl LocalStorage {
    pub fn new(p: &str) -> LocalStorage {
        let path = PathBuf::from(p);
        // Creating necessary directories if they don't exist
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create data directory");
        }
        if !path.join(MUTABLE_LOGS_FILE_DIRECTORY).exists() {
            fs::create_dir_all(path.join(MUTABLE_LOGS_FILE_DIRECTORY)).expect("Failed to create md directory");
        }

        if !path.join(MUTABLE_TRACES_FILE_DIRECTORY).exists() {
            fs::create_dir_all(path.join(MUTABLE_TRACES_FILE_DIRECTORY)).expect("Failed to create md_traces directory");
        }
        LocalStorage { path }
    }
}

impl Storage for LocalStorage {
    fn read_config(&self) -> Option<Vec<u8>> {
        // Implement reading config from local storage
        let ini_file_path = self.path.join("ini.ry");
        println!("Reading config from {}", ini_file_path.display());
        let buffer = fs::read(&ini_file_path).ok();
        match buffer {
            Some(buffer) => {
                println!("Read config from {} Done!", ini_file_path.display());
                Some(buffer)
            },
            None => {
                println!("Config file not found at {}, initializing with default config.", ini_file_path.display());
                None
            }
        }
    }

    fn write_config(&self, config: Vec<u8>) {
        // Implement writing config to local storage
        let ini_file_path = self.path.join("ini.ry");
        let mut file = OpenOptions::new().write(true).create(true).open(&ini_file_path).unwrap();
        let status = file.write_all(&config);
        println!("Wrote config to {:?} Done!", status);
    }
    
    fn write(&self, file_name: &str, data: Vec<u8>, offset: Option<usize>) {
        let file_path = self.path.join(file_name);
        let mut file = OpenOptions::new().write(true).create(true).open(&file_path).unwrap();
        if let Some(offset) = offset {
            file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        }
        file.write_all(&data).unwrap();
    }
    
    fn read(&self, file_name: &str, offset: Option<usize>) -> Vec<u8> {
        let file_path = self.path.join(file_name);
        let mut file = OpenOptions::new().read(true).open(&file_path).unwrap();
        if let Some(offset) = offset {
            file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        }
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }
 
}
