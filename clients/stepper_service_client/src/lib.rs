use client::ServiceClient;
use ipc::{IpcReader, IpcWriter};
use stepper_motor_data::{StepperMotorRequest, StepperMotorResponse};

pub type RequestSerializer = dyn DataTransformer<StepperMotorRequest, Vec<u8>, String>;
pub type ResponseParser = dyn DataTransformer<Vec<u8>, StepperMotorResponse, String>;
pub type RawDataReader = dyn IpcReader<Vec<u8>, String>;
pub type RawDataWriter = dyn IpcWriter<Vec<u8>, String>;

pub use default_transformers::{JsonRequestSerializer, JsonResponseParser};

pub struct StepperServiceClient {
    raw_data_reader:        Box<RawDataReader>,
    raw_data_writer:        Box<RawDataWriter>,
    request_serializer:     Box<RequestSerializer>,
    response_parser:        Box<ResponseParser>,
}

impl StepperServiceClient {
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

impl ServiceClient<StepperMotorRequest, StepperMotorResponse, String> for StepperServiceClient {
    fn run_request(&mut self, request: &StepperMotorRequest) -> Result<StepperMotorResponse, String> {
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
    use std::time::Duration;

    use serde_json::{json, Value};
    use stepper_motor_data::StepperMotorDirection;

    use super::*;

    #[test]
    fn client_new_sanity() {
        // GIVEN
        let test_request = StepperMotorRequest {
            motor_id: "test_motor".to_string(),
            direction: StepperMotorDirection::CCW,
            steps_number: 15,
            step_duration: Duration::from_millis(1234),
        };
        let expected_response = StepperMotorResponse::SUCCESS;

        // THEN
        let _ = StepperServiceClient::new(
            Box::new(TestIpcReader {
                expected_response,
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
        let test_request = StepperMotorRequest {
            motor_id: "test_motor".to_string(),
            direction: StepperMotorDirection::CCW,
            steps_number: 15,
            step_duration: Duration::from_millis(1234),
        };
        let expected_response = StepperMotorResponse::SUCCESS;
        
        // WHEN
        let mut instance = StepperServiceClient::new(
            Box::new(TestIpcReader {
                expected_response: expected_response.clone(),
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
        expected_response: StepperMotorResponse,
    }

    impl IpcReader<Vec<u8>, String> for TestIpcReader {
        fn read_data(&mut self) -> Result<Vec<u8>, String> {
            let json_val = match &self.expected_response {
                StepperMotorResponse::SUCCESS => json!({
                    "result": "SUCCESS",
                }),
                StepperMotorResponse::FAILURE(msg) => json!({
                    "result": "FAILURE",
                    "whar": msg,
                }),
            };
            let str_data = serde_json::to_string(&json_val).unwrap();
            Ok(str_data.as_bytes().to_vec())
        }
    }

    struct TestIpcWriter {
        test_request: StepperMotorRequest,
    }

    impl IpcWriter<Vec<u8>, String> for TestIpcWriter {
        fn write_data(&mut self, data: &Vec<u8>) -> Result<(), String> {
            let json_data: Value = serde_json::from_slice(data).unwrap();
            assert_eq!(&self.test_request.motor_id, json_data.get("motor_id").unwrap().as_str().unwrap());
            Ok(())
        }
    }
}