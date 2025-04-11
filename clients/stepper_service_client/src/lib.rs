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