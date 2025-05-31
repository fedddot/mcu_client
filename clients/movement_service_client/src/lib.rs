use ipc::{IpcReader, IpcWriter};

pub use client::ServiceClient;
pub use movement_data::{MovementApiRequest, MovementApiResponse};

pub type RequestSerializer = dyn DataTransformer<MovementApiRequest, Vec<u8>, String>;
pub type ResponseParser = dyn DataTransformer<Vec<u8>, MovementApiResponse, String>;
pub type RawDataReader = dyn IpcReader<Vec<u8>, String>;
pub type RawDataWriter = dyn IpcWriter<Vec<u8>, String>;

pub use default_transformers::{JsonRequestSerializer, JsonResponseParser};

pub struct MovementServiceClient {
    raw_data_reader:        Box<RawDataReader>,
    raw_data_writer:        Box<RawDataWriter>,
    request_serializer:     Box<RequestSerializer>,
    response_parser:        Box<ResponseParser>,
}

impl MovementServiceClient {
    pub fn new(
        raw_data_reader:        Box<RawDataReader>,
        raw_data_writer:        Box<RawDataWriter>,
        request_serializer:     Box<RequestSerializer>,
        response_parser:        Box<ResponseParser>,
    ) -> Self {
        Self {
            raw_data_reader,
            raw_data_writer,
            request_serializer,
            response_parser,
        }
    }
}

impl ServiceClient<MovementApiRequest, MovementApiResponse, String> for MovementServiceClient {
    fn run_request(&mut self, request: &MovementApiRequest) -> Result<MovementApiResponse, String> {
        let serial_request = self.request_serializer.transform(request)?;
        self.raw_data_writer.write_data(&serial_request)?;
        let serial_response = self.raw_data_reader.read_data()?;
        let response = self.response_parser.transform(&serial_response)?;
        Ok(response)
    }
}

pub trait DataTransformer<Input, Output, Error> {
    fn transform(&self, input: &Input) -> Result<Output, Error>;
}

mod default_transformers;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use mockall::mock;
    use serde_json::json;
    use movement_data::Vector;

    #[test]
    fn client_new_sanity() {
        // GIVEN
        let test_raw_data_reader = MockIpcReader::default();
        let test_raw_data_writer = MockIpcWriter::default();

        // THEN
        let _ = MovementServiceClient::new(
            Box::new(test_raw_data_reader),
            Box::new(test_raw_data_writer),
            Box::new(JsonRequestSerializer),
            Box::new(JsonResponseParser),
        );
    }

    #[test]
    fn client_run_request_sanity() {
        // GIVEN
        let test_config_req = MovementApiRequest::Config {
            axes_configs: HashMap::new(),
        };
        let test_linear_mvmnt_req = MovementApiRequest::LinearMovement {
            destination: Vector::new(1.0, 2.0, 3.0),
            speed: 4.0,
        };
        let mut test_raw_data_reader = MockIpcReader::default();
        test_raw_data_reader
            .expect_read_data()
            .returning(move || {
                let json_response = json!({
                    "result": "SUCCESS",
                });
                let serial_response = serde_json::to_vec(&json_response).unwrap();
                Ok(serial_response)
            });
        let mut test_raw_data_writer = MockIpcWriter::default();
        test_raw_data_writer
            .expect_write_data()
            .returning(|data| {         
                println!("Writing data: {:?}", std::str::from_utf8(data).unwrap());
                Ok(())
            });
        // WHEN
        let mut client = MovementServiceClient::new(
            Box::new(test_raw_data_reader),
            Box::new(test_raw_data_writer),
            Box::new(JsonRequestSerializer),
            Box::new(JsonResponseParser),
        );

        // THEN
        let response = client.run_request(&test_linear_mvmnt_req);
        assert!(response.is_ok());

        let response = client.run_request(&test_config_req);
        assert!(response.is_ok());
    }

    mock! {
        pub IpcReader {}
        impl IpcReader<Vec<u8>, String> for IpcReader {
            fn read_data(&mut self) -> Result<Vec<u8>, String>;
        }
    }
    mock! {
        pub IpcWriter {}
        impl IpcWriter<Vec<u8>, String> for IpcWriter {
            fn write_data(&mut self, data: &Vec<u8>) -> Result<(), String>;
        }
    }
}