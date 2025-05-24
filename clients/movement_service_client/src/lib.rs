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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use serde_json::{json, Value};

//     #[test]
//     fn client_new_sanity() {
//         // GIVEN
//         let test_request = MovementApiRequest {
//             movement_type: MovementType::Linear(
//                 LinearMovementData {
//                     destination: Vector::new(1.0, 2.0, 3.0),
//                     speed: 4.0,
//                 }
//             )
//         };
//         let expected_response = ResultCode::Ok;

//         // THEN
//         let _ = MovementServiceClient::new(
//             Box::new(TestIpcReader {
//                 expected_response,
//                 expected_message: None,
//             }),
//             Box::new(TestIpcWriter {
//                 test_request,
//             }),
//             Box::new(JsonRequestSerializer),
//             Box::new(JsonResponseParser),
//         );
//     }
// }