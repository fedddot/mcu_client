use std::time::Duration;
use std::sync::{Arc, Mutex, MutexGuard};

use serialport::{DataBits, FlowControl, Parity, StopBits};

pub use serialport::SerialPort;

#[derive(Clone)]
pub struct UartPort {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

#[derive(Clone)]
pub struct UartPortConfig {
    pub port_name: String,
    pub baud: u32,
    pub timeout: Duration,
}

impl UartPort {
    pub fn new(config: &UartPortConfig) -> Result<Self, String> {
        let port_res = serialport::new(&config.port_name, config.baud)
            .timeout(config.timeout)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .parity(Parity::None)
            .flow_control(FlowControl::None)
            .open();
        match port_res {
            Ok(port) => Ok(Self {
                port: Arc::new(Mutex::new(port)),
            }),
            Err(err) => Err(format!("failed to open serial port {}: {}", config.port_name, err)),
        }
    }

    pub fn get_mut(&mut self) -> Result<MutexGuard<Box<dyn SerialPort>>, String> {
        match self.port.lock() {
            Ok(guard) => Ok(guard),
            Err(err) => Err(err.to_string()),
        }
    }
}