use ipc::{IpcReader, IpcWriter};

pub use client::ServiceClient;
pub use movement_data::{MovementManagerRequest, MovementManagerResponse};
pub type RequestSerializer = dyn DataTransformer<MovementManagerRequest, Vec<u8>, String>;
pub type ResponseParser = dyn DataTransformer<Vec<u8>, MovementManagerResponse, String>;
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

impl ServiceClient<MovementManagerRequest, MovementManagerResponse, String> for MovementServiceClient {
    fn run_request(&mut self, request: &MovementManagerRequest) -> Result<MovementManagerResponse, String> {
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
    use movement_data::{LinearMovementData, MovementType, ResultCode, Vector};
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn client_new_sanity() {
        // GIVEN
        let test_request = MovementManagerRequest {
            movement_type: MovementType::Linear(
                LinearMovementData {
                    destination: Vector::new(1.0, 2.0, 3.0),
                    speed: 4.0,
                }
            )
        };
        let expected_response = ResultCode::Ok;

        // THEN
        let _ = MovementServiceClient::new(
            Box::new(TestIpcReader {
                expected_response,
                expected_message: None,
            }),
            Box::new(TestIpcWriter {
                test_request,
            }),
            Box::new(JsonRequestSerializer),
            Box::new(JsonResponseParser),
        );
    }

    #[test]
    fn client_run_request_sanity() {
        // GIVEN
        let test_request = MovementManagerRequest {
            movement_type: MovementType::Linear(
                LinearMovementData {
                    destination: Vector::new(1.0, 2.0, 3.0),
                    speed: 4.0,
                }
            )
        };
        let expected_response = ResultCode::Ok;
        let expected_message = "test message";
        
        // WHEN
        let mut instance = MovementServiceClient::new(
            Box::new(TestIpcReader {
                expected_response: expected_response.clone(),
                expected_message: Some(expected_message.to_string()),
            }),
            Box::new(TestIpcWriter {
                test_request: test_request.clone(),
            }),
            Box::new(JsonRequestSerializer),
            Box::new(JsonResponseParser),
        );

        // THEN
        let result = instance.run_request(&test_request);
        assert!(result.is_ok());
    }

    struct TestIpcReader {
        expected_response: ResultCode,
        expected_message: Option<String>,
    }

    impl IpcReader<Vec<u8>, String> for TestIpcReader {
        fn read_data(&mut self) -> Result<Vec<u8>, String> {
            let mut json_val = Value::default();
            match &self.expected_response {
                ResultCode::Ok => json_val["result"] = json!(0),
                ResultCode::BadRequest => json_val["result"] = json!(1),
                ResultCode::Exception => json_val["result"] = json!(2),
            };
            if let Some(message) = &self.expected_message {
                json_val["message"] = json!(message);
            }
            let str_data = serde_json::to_string(&json_val).unwrap();
            Ok(str_data.as_bytes().to_vec())
        }
    }

    struct TestIpcWriter {
        test_request: MovementManagerRequest,
    }

    impl IpcWriter<Vec<u8>, String> for TestIpcWriter {
        fn write_data(&mut self, data: &Vec<u8>) -> Result<(), String> {
            let json_data: Value = serde_json::from_slice(data).unwrap();
            println!("TestIpcWriter: json data: {:?}, test_request: {:?}", json_data, self.test_request);
            Ok(())
        }
    }
}