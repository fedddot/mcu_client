use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GcodeProcessorConfig {
    pub uart_port: UartPortConfig,
    pub uart_package: UartPackageConfig,
    pub state_storage: GcodeProcessorStorageConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UartPortConfig {
    pub port_name: String,
    pub baud: u32,
    pub response_timeout_s: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UartPackageConfig {
    pub preamble: String,
    pub size_field_length: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GcodeProcessorStorageConfig {
    pub file_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let test_cfg = GcodeProcessorConfig {
            uart_port: UartPortConfig {
                port_name: "/dev/ttyACM0".into(),
                baud: 115200,
                response_timeout_s: 60
            },
            uart_package: UartPackageConfig {
                preamble: "MSG_PREAMBLE".into(),
                size_field_length: 4,
            },
            state_storage: GcodeProcessorStorageConfig {
                file_path: "/usr/app/src/target/state.json".into(),
            },
        };

        // WHEN
        let test_cfg_serial = serde_json::to_string(&test_cfg);
        assert!(test_cfg_serial.is_ok());
        let test_cfg_serial = test_cfg_serial.unwrap();
        println!("serialized: {test_cfg_serial}");
    }
}