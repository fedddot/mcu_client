use ipc::{IpcReader, IpcWriter};

pub use client::ServiceClient;
pub use stepper_data::{
    StepperApiRequest,
    StepperApiResponse,
    RequestType,
    ResultCode,
};

pub type RequestSerializer = dyn DataTransformer<StepperApiRequest, Vec<u8>, String>;
pub type ResponseParser = dyn DataTransformer<Vec<u8>, StepperApiResponse, String>;
pub type RawDataReader = dyn IpcReader<Vec<u8>, String>;
pub type RawDataWriter = dyn IpcWriter<Vec<u8>, String>;

pub use proto_transformers::{ProtoRequestSerializer, ProtoResponseParser};

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

impl ServiceClient<StepperApiRequest, StepperApiResponse, String> for StepperServiceClient {
    fn run_request(&mut self, request: &StepperApiRequest) -> Result<StepperApiResponse, String> {
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

mod proto_transformers;

#[cfg(test)]
mod test {
    use crate::proto_transformers::pb;

    use super::*;
    use mockall::mock;
    use prost::Message;

    #[test]
    fn client_new_sanity() {
        // GIVEN
        let test_raw_data_reader = MockIpcReader::default();
        let test_raw_data_writer = MockIpcWriter::default();

        // THEN
        let _ = StepperServiceClient::new(
            Box::new(test_raw_data_reader),
            Box::new(test_raw_data_writer),
            Box::new(ProtoRequestSerializer),
            Box::new(ProtoResponseParser),
        );
    }

    #[test]
    fn client_run_request_sanity() {
        // GIVEN
        let test_get_req = StepperApiRequest {
            request_type: RequestType::GetTemperature,
            set_temperature: None,
            time_resolution_ms: None,
        };
        let mut test_raw_data_reader = MockIpcReader::default();
        test_raw_data_reader
            .expect_read_data()
            .returning(move || {
                let pb_response = pb::StepperApiResponse {
                    status: pb::StatusCode::Success as i32,
                    message: "Temperature retrieved successfully".into(),
                    current_temperature: 22.5,
                };
                let serial_response = pb_response.encode_to_vec();
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
        let mut client = StepperServiceClient::new(
            Box::new(test_raw_data_reader),
            Box::new(test_raw_data_writer),
            Box::new(ProtoRequestSerializer),
            Box::new(ProtoResponseParser),
        );

        // THEN
        let response = client.run_request(&test_get_req);
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