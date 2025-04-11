use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

pub trait SizeEncoder {
    fn encode(&self, size: usize) -> Result<Vec<u8>, String>;
}

pub trait SizeDecoder {
    fn raw_data_size(&self) -> usize;
    fn decode(&self, raw_data: &[u8]) -> usize;
}

pub struct UartMcuClient {
    port: Box<dyn SerialPort>,
    preamble: Vec<u8>,
    size_encoder: ,
}

impl UartMcuClient {
    pub fn new(
        uart_path: &str,
        baud: u32,
        timeout_ms: u32,
        preamble: &[u8],
        encoded_size_length: usize,
    ) -> Self {
        let port: Box<dyn SerialPort> = serialport::new(uart_path, baud)
            .timeout(Duration::from_millis(timeout_ms as u64))
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .parity(Parity::None)
            .flow_control(FlowControl::None)
            .open()
            .unwrap_or_else(|e| panic!("failed to open serial port on {}: {}", uart_path, e));
        Self {
            port,
            preamble: preamble.to_vec(),
            encoded_size_length,
        }
    }

    pub fn send_package(&mut self, package: &[u8]) -> Result<(), String> {
        let mut package_data = vec![];
        package_data.extend_from_slice(&self.preamble);
        let encoded_size = Self::encode_msg_size(package.len(), self.encoded_size_length);
        package_data.extend_from_slice(&encoded_size);
        package_data.extend_from_slice(package);
        if self.port.write_all(&package_data).is_err() {
            return Err("failed to send package data".to_string());
        }
        Ok(())
    }

    pub fn read_package(&mut self) -> Result<Vec<u8>, String> {
        let preamble_size = self.preamble.len();
        let header_size = preamble_size + self.encoded_size_length;
        let mut header_buff = vec![0; header_size];
        if self.port.read_exact(header_buff.as_mut_slice()).is_err() {
            return Err("failed to read package header".to_string());
        }
        if self.preamble != header_buff[0..preamble_size] {
            return Err("invalid preamble received".to_string());
        }
        let package_size = Self::decode_msg_size(&header_buff[preamble_size..(preamble_size + self.encoded_size_length)]);
        let mut data_buff = vec![0; package_size];
        if self.port.read_exact(data_buff.as_mut_slice()).is_err() {
            return Err("failed to read package data".to_string());
        }
        Ok(data_buff)
    }

    fn encode_msg_size(msg_size: usize, encoded_size_length: usize) -> Vec<u8> {
        const BITS_IN_BYTE: usize = 8;
        let mut encoded_size = vec![0; encoded_size_length];
        encoded_size.iter_mut().enumerate().for_each(
            |(i, c)| {
                let less_significant_byte = ((msg_size >> (BITS_IN_BYTE * i)) & 0xFF) as u8;
                *c = less_significant_byte;
            }
        );
        encoded_size.to_vec()
    }

    fn decode_msg_size(msg_size_serial: &[u8]) -> usize {
        const BITS_IN_BYTE: usize = 8;
        let mut decoded_size: usize = 0;
        for &byte in msg_size_serial.iter().rev() {
            decoded_size <<= BITS_IN_BYTE;
            decoded_size |= byte as usize;
        }
        decoded_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sanity() {
        // GIVEN
        let uart_path = "/dev/ttyACM0";
        let uart_baud = 115200;
        let uart_timeout_ms = 3000;
        let test_preamble = "MSG_PREAMBLE";
        let test_package = "bla-bla";
        let encoded_size_len = 4;

        // WHEN
        let mut instance = UartMcuClient::new(
            uart_path,
            uart_baud,
            uart_timeout_ms,
            test_preamble.as_bytes(),
            encoded_size_len,
        );

        // THEN
        let send_result = instance.send_package(test_package.as_bytes());
        assert!(send_result.is_ok());

        let read_result = instance.read_package();
        assert!(read_result.is_ok());
    }

    #[test]
    fn encode_decode_sanity() {
        // GIVEN
        let encoded_size_len = 4;
        let test_sizes = [u32::MIN as usize, 10, 1234, 145670, u32::MAX as usize];

        // THEN
        test_sizes.iter().for_each(
            |s| {
                let encoded_size = UartMcuClient::encode_msg_size(*s, encoded_size_len);
                let decoded_size = UartMcuClient::decode_msg_size(&encoded_size);
                assert_eq!(*s, decoded_size);
            }
        );
    }
}
