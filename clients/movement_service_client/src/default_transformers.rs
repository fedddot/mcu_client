use serde_json::{json, Map, Value};

use movement_data::*;

pub use crate::DataTransformer;

pub struct JsonRequestSerializer;

impl JsonRequestSerializer {
    fn serialize_request_type(api_request: &MovementApiRequest) -> Value {
        match api_request {
            MovementApiRequest::Config { .. } => json!("CONFIG"),
            MovementApiRequest::LinearMovement { .. } => json!("LINEAR_MOVEMENT"),
            MovementApiRequest::RotationalMovement { .. } => json!("ROTATIONAL_MOVEMENT"),
        }
    }

    fn serialize_request_data(api_request: &MovementApiRequest) -> Value {
        let request_str = serde_json::to_string(api_request).unwrap();
        let request_data: Map<String, Value> = serde_json::from_str(&request_str).unwrap();
        let request_data: Vec<Value> = request_data
            .iter()
            .map(|(_, v)| v.clone())
            .collect();
        if request_data.len() != 1 {
            panic!("expected request json data to have exactly one item");
        }
        request_data[0].clone()
    }
}

impl DataTransformer<MovementApiRequest, Vec<u8>, String> for JsonRequestSerializer {
    fn transform(&self, input: &MovementApiRequest) -> Result<Vec<u8>, String> {
        let mut json_val = Self::serialize_request_data(input);
        json_val["request_type"] = Self::serialize_request_type(input);
        let json_string = match serde_json::to_string(&json_val) {
            Ok(str_val) => str_val,
            Err(err) => return Err(err.to_string()),
        };
        Ok(json_string.into_bytes())
    }
}

pub struct JsonResponseParser;

impl JsonResponseParser {
    fn parse_result(json_data: &Value) -> Result<StatusCode, String> {
        let Some(status) = json_data.get("status") else {
            return Err("missing status field".to_string());
        };
        let Some(status) = status.as_str() else {
            return Err("result field has wrong format".to_string());
        };
        match status {
            "SUCCESS" => Ok(StatusCode::Success),
            "FAILURE" => Ok(StatusCode::Error),
            _ => Err(format!("unsupported result value: {}", status)),
        }
    }

    fn parse_message(json_data: &Value) -> Result<Option<String>, String> {
        let message_opt = json_data.get("message");
        if message_opt.is_none() {
            return Ok(None);
        }
        let Some(message) = message_opt.unwrap().as_str() else {
            return Err("message field has wrong format".to_string());
        };
        Ok(Some(message.to_string()))
    }
}

impl DataTransformer<Vec<u8>, MovementApiResponse, String> for JsonResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<MovementApiResponse, String> {
        let parsed_report: MovementApiResponse = serde_json::from_slice(input).map_err(|err| err.to_string())?;
        Ok(parsed_report)
    }
}

#[cfg(test)]
mod test {
    use movement_data::Vector;

    use super::*;

    #[test]
    fn json_request_ser_sanity() {
        // GIVEN
        let test_request = MovementApiRequest::LinearMovement(LinearMovement {
            destination: Vector::new(1.0, 2.0, 3.0),
            speed: 4.9,
        });
        let mut expected_value = JsonRequestSerializer::serialize_request_data(&test_request);
        expected_value["request_type"] = JsonRequestSerializer::serialize_request_type(&test_request);

        // WHEN
        let request_serializer = JsonRequestSerializer;

        // THEN
        let serial_request = request_serializer.transform(&test_request).unwrap();
        let parsed_serial_request: Value = serde_json::from_slice(&serial_request).unwrap();
        assert_eq!(expected_value, parsed_serial_request);
    }
}