use thermo_data::{
    ThermostatApiRequest,
    ThermostatApiResponse,
    StatusCode,
    RequestType,
};
use prost::Message;

use crate::DataTransformer;

fn serialize_thermostat_request(request: &ThermostatApiRequest) -> Vec<u8> {
    let pb_request_type = match request.request_type {
        RequestType::Start => pb::RequestType::Start as i32,
        RequestType::Stop => pb::RequestType::Stop as i32,
        RequestType::GetTemperature => pb::RequestType::GetTemp as i32,
    };
    let pb_request = pb::ThermostatApiRequest {
        request_type: pb_request_type,
        set_temperature: request.set_temperature.unwrap_or(0.0),
        time_resolution_ms: request.time_resolution_ms.unwrap_or(0),
    };
    pb_request.encode_to_vec()
}

fn parse_thermostat_response(data: &[u8]) -> Result<ThermostatApiResponse, String> {
    let pb_response = pb::ThermostatApiResponse::decode(data)
        .map_err(|e| format!("failed to decode response: {e}"))?;
    let pb_status = pb::StatusCode::try_from(pb_response.status)
        .map_err(|e| format!("failed to convert status code: {e}"))?;
    let status = match pb_status {
        pb::StatusCode::Success => StatusCode::Success,
        pb::StatusCode::Failure => StatusCode::Failure,
    };
    Ok(ThermostatApiResponse {
        status,
        message: pb_response.message.into(),
        current_temperature: Some(pb_response.current_temperature),
    })
}

pub struct ProtoRequestSerializer;

impl DataTransformer<ThermostatApiRequest, Vec<u8>, String> for ProtoRequestSerializer {
    fn transform(&self, input: &ThermostatApiRequest) -> Result<Vec<u8>, String> {
        Ok(serialize_thermostat_request(input))
    }
}

pub struct ProtoResponseParser;

impl DataTransformer<Vec<u8>, ThermostatApiResponse, String> for ProtoResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<ThermostatApiResponse, String> {
        parse_thermostat_response(input)
    }
}

pub(crate) mod pb {
    tonic::include_proto!("service_api");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let test_request = ThermostatApiRequest {
            request_type: RequestType::Start,
            set_temperature: Some(22.5),
            time_resolution_ms: Some(1000),
        };

        // THEN
        let serialized = serialize_thermostat_request(&test_request);
        println!("Serialized request: {serialized:?}");
    }
}
