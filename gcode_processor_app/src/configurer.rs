pub use crate::config::GcodeProcessorConfig;

pub struct JsonFileConfigurer {
    file_path: String,
}

impl JsonFileConfigurer {
    pub fn new(file_path: &str) -> Self {
        Self { file_path: file_path.into() }
    }

    pub fn config(&self) -> Result<GcodeProcessorConfig, String> {
        let file = std::fs::File::open(&self.file_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_reader(file)
            .map_err(|e| e.to_string())
    }
}

