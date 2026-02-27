pub trait Storage {
    fn write(&self, file_name: &str, data: Vec<u8>, offset: Option<usize>);

    fn read(&self, file_name: &str, offset: Option<usize>, size: Option<usize>) -> Option<Vec<u8>>;

    fn delete(&self, file_name: &str);

    fn is_file_exists(&self, file_name: &str) -> bool;

}