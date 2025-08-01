use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThermostatConfig {
    pub uart_port: UartPortConfig,
    pub uart_package: UartPackageConfig,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let test_cfg = ThermostatConfig {
            uart_port: UartPortConfig {
                port_name: "/dev/ttyACM0".into(),
                baud: 115200,
                response_timeout_s: 60
            },
            uart_package: UartPackageConfig {
                preamble: "MSG_PREAMBLE".into(),
                size_field_length: 4,
            },
        };

        // WHEN
        let test_cfg_serial = serde_json::to_string(&test_cfg);
        assert!(test_cfg_serial.is_ok());
        let test_cfg_serial = test_cfg_serial.unwrap();
        println!("serialized: {test_cfg_serial}");
    }
}