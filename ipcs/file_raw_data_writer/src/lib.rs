use std::path::PathBuf;

pub type FileRawData = Vec<u8>;

pub struct FileRawDataWriter {
    file_path: PathBuf,
    movement_app_cli_path: PathBuf,
}

impl FileRawDataWriter {
    pub fn new(file_path: &str, movement_app_cli_path: &str) -> Self {
        Self {
            file_path: PathBuf::from(file_path),
            movement_app_cli_path: PathBuf::from(movement_app_cli_path),
        }
    }
    
}

impl ipc::IpcWriter<FileRawData, String> for FileRawDataWriter {
    fn write_data(&mut self, data: &FileRawData) -> Result<(), String> {
        todo!("write the data to the file");
        todo!("and call the movement app cli to process it");
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
