use std::path::PathBuf;

pub type FileRawData = Vec<u8>;

pub struct FileRawDataWriter {
    file_path: PathBuf,
}

impl FileRawDataWriter {
    pub fn new(file_path: &str) -> Self {
        Self { file_path: PathBuf::from(file_path) }
    }
    
}

impl ipc::IpcWriter<FileRawData, String> for FileRawDataWriter {
    fn write_data(&mut self, data: &FileRawData) -> Result<(), String> {
        todo!("implement write_data");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!();
    }
}
