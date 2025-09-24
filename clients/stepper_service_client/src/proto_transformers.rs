use stepper_data::{
    StepperApiRequest,
    StepperApiResponse,
    RequestType,
    ResultCode,
};
use prost::Message;

use crate::DataTransformer;

fn serialize_stepper_request(request: &StepperApiRequest) -> Vec<u8> {
    match &request.request_type {
        RequestType::Enable => {
            let pb_request = pb::StepperRequest {
                request: Some(pb::stepper_request::Request::EnableRequest(
                    pb::StepperEnableRequest {
                        status: pb::StepperStatus::Enabled as i32,
                    },
                )),
            };
            pb_request.encode_to_vec()
        }
        RequestType::Disable => {
            let pb_request = pb::StepperRequest {
                request: Some(pb::stepper_request::Request::EnableRequest(
                    pb::StepperEnableRequest {
                        status: pb::StepperStatus::Disabled as i32,
                    },
                )),
            };
            pb_request.encode_to_vec()
        },
        unsupported => panic!("unsupported request type: {unsupported:?}"),
    }
}

fn parse_stepper_response(data: &[u8]) -> Result<StepperApiResponse, String> {
    let pb_response = pb::StepperResponse::decode(data)
        .map_err(|e| format!("failed to decode response: {e}"))?;
    let pb_result = pb::StepperResultCode::try_from(pb_response.result)
        .map_err(|e| format!("failed to convert result code: {e}"))?;
    let pb_status = pb::StepperStatus::try_from(pb_response.stepper_status)
        .map_err(|e| format!("failed to convert status code: {e}"))?;
    let result = match pb_result {
        pb::StepperResultCode::Success => ResultCode::Success,
        pb::StepperResultCode::Failure => ResultCode::Failure,
    };
    let status = match pb_status {
        pb::StepperStatus::Disabled => stepper_data::Status::Disabled,
        pb::StepperStatus::Enabled => stepper_data::Status::Enabled,
    };
    Ok(StepperApiResponse {
        result,
        message: pb_response.error_message.into(),
        status: Some(status),
    })
}

pub struct ProtoRequestSerializer;

impl DataTransformer<StepperApiRequest, Vec<u8>, String> for ProtoRequestSerializer {
    fn transform(&self, input: &StepperApiRequest) -> Result<Vec<u8>, String> {
        Ok(serialize_stepper_request(input))
    }
}

pub struct ProtoResponseParser;

impl DataTransformer<Vec<u8>, StepperApiResponse, String> for ProtoResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<StepperApiResponse, String> {
        parse_stepper_response(input)
    }
}

pub(crate) mod pb {
    tonic::include_proto!("stepper_service");
}