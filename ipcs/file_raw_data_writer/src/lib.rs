use serde_json::{json, Value};

pub type FileRawData = Vec<u8>;

pub struct FileRawDataWriter {
    file_path: String,
}

impl FileRawDataWriter {
    pub fn new(file_path: &str) -> Self {
        Self { file_path: String::from(file_path) }
    }
}

impl ipc::IpcWriter<FileRawData, String> for FileRawDataWriter {
    fn write_data(&mut self, data: &FileRawData) -> Result<(), String> {
        let json_data: Value = serde_json::from_slice(data).map_err(|e| e.to_string())?;
        let json_data = json!([ json_data ]);
        let serialized = serde_json::to_vec(&json_data).map_err(|e| e.to_string())?;
        std::fs::write(&self.file_path, &serialized).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use ipc::IpcWriter;

    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let tmp_dir = tempfile::tempdir().unwrap();
        let test_file_path = tmp_dir.path().join("test.json");
        let test_data = "{\"key\": \"value\"}"
            .as_bytes()
            .to_vec();

        // WHEN
        let mut writer = FileRawDataWriter::new(test_file_path.to_str().unwrap());
        
        // THEN
        let result = writer.write_data(&test_data);
        assert!(result.is_ok(), "Failed to write data: {}", result.unwrap_err());
        let file_contents = std::fs::read_to_string(test_file_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&file_contents).unwrap();
        let arr = parsed.as_array().expect("Expected a JSON array");
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().expect("Expected a JSON object");
        assert_eq!(obj.get("key").unwrap(), "value");
    }
}
