pub use ipc::{IpcReader, IpcWriter};
pub use uart_port::UartPort;
pub use default_size_encoding::{DefaultSizeDecoder, DefaultSizeEncoder};

pub struct UartSizedPackageReader {
    port: UartPort,
    preamble: Vec<u8>,
    size_decoder: Box<dyn SizeDecoder + Send + Sync>,
}

impl UartSizedPackageReader {
    pub fn new(
        port: &UartPort,
        preamble: &[u8],
        size_decoder: Box<dyn SizeDecoder + Send + Sync>,
    ) -> Self {
        Self {
            port: port.clone(),
            preamble: preamble.to_vec(),
            size_decoder,
        }
    }
}

pub trait SizeDecoder {
    fn raw_data_size(&self) -> usize;
    fn decode(&self, raw_data: &[u8]) -> Result<usize, String>;
}

impl IpcReader<Vec<u8>, String> for UartSizedPackageReader {
    fn read_data(&mut self) -> Result<Vec<u8>, String> {
        let preamble_size = self.preamble.len();
        let encoded_size_length = self.size_decoder.raw_data_size();
        let header_size = preamble_size + encoded_size_length;
        
        let mut port_handle = self.port.get_mut()?;
        
        let mut header_buff = vec![0; header_size];
        let read_result = port_handle.read_exact(header_buff.as_mut_slice());
        if read_result.is_err() {
            return Err(format!("failed to read package header: {read_result:?}"));
        }
        if self.preamble != header_buff[0..preamble_size] {
            return Err(format!("invalid preamble received: {:?}", &header_buff[0..preamble_size]));
        }
        let package_size = self.size_decoder.decode(&header_buff[preamble_size..(preamble_size + encoded_size_length)])?;
        let mut data_buff = vec![0; package_size];
        if port_handle.read_exact(data_buff.as_mut_slice()).is_err() {
            return Err("failed to read package data".to_string());
        }
        Ok(data_buff)
    }
}

pub struct UartSizedPackageWriter {
    port: UartPort,
    preamble: Vec<u8>,
    size_encoder: Box<dyn SizeEncoder + Send + Sync>,
}


impl UartSizedPackageWriter {
    pub fn new(
        port: &UartPort,
        preamble: &[u8],
        size_encoder: Box<dyn SizeEncoder + Send + Sync>,
    ) -> Self {
        Self {
            port: port.clone(),
            preamble: preamble.to_vec(),
            size_encoder,
        }
    }
}

pub trait SizeEncoder {
    fn encode(&self, size: usize) -> Result<Vec<u8>, String>;
}

impl IpcWriter<Vec<u8>, String> for UartSizedPackageWriter {
    fn write_data(&mut self, data: &Vec<u8>) -> Result<(), String> {
        let mut package_data = vec![];
        package_data.extend_from_slice(&self.preamble);
        let encoded_size = self.size_encoder.encode(data.len())?;
        package_data.extend_from_slice(&encoded_size);
        package_data.extend_from_slice(data);
        let mut port_handle = self.port.get_mut()?;
        if port_handle.write_all(&package_data).is_err() {
            return Err("failed to send package data".to_string());
        }
        Ok(())
    }
}

mod default_size_encoding;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    #[ignore = "WITH MCU CONNECTED ONLY"]
    fn new_sanity() {
        // GIVEN
        let port_name = "/dev/ttyACM0";
        let test_preamble = "MSG_PREAMBLE";
        let test_package = "{\"direction\": 0, \"steps_number\": 1000, \"step_duration_ms\": 10}";
        let encoded_size_len = 4;

        // WHEN
        let uart_port = generate_uart(port_name);
        let mut reader = UartSizedPackageReader::new(
            &uart_port,
            test_preamble.as_bytes(),
            Box::new(DefaultSizeDecoder::new(encoded_size_len)),
        );
        let mut writer = UartSizedPackageWriter::new(
            &uart_port,
            test_preamble.as_bytes(),
            Box::new(DefaultSizeEncoder::new(encoded_size_len)),
        );

        // THEN
        let send_result = writer.write_data(&test_package.as_bytes().to_vec());
        assert!(send_result.is_ok());

        let read_result = reader.read_data();
        assert!(read_result.is_ok());
        let read_data = read_result.unwrap();
        println!("Received data: {:?}", String::from_utf8(read_data).unwrap());
    }

    fn generate_uart(port_name: &str) -> UartPort {
        UartPort::new(
            port_name,
            115200,
            Duration::from_millis(3000),
        )
        .unwrap()
    }
}
