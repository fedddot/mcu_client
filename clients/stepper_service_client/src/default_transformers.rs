use serde_json::{json, Value};

use crate::{DataTransformer, StepperMotorRequest, StepperMotorResponse, StepperMotorResponseCode, StepperMotorState, StepperMotorDirection};

pub struct JsonRequestSerializer;

impl JsonRequestSerializer {
    fn serialize_direction(direction: &StepperMotorDirection) -> Value {
        match direction {
            StepperMotorDirection::CCW => json!(0),
            StepperMotorDirection::CW => json!(1),
        }
    }
}

impl DataTransformer<StepperMotorRequest, Vec<u8>, String> for JsonRequestSerializer {
    fn transform(&self, input: &StepperMotorRequest) -> Result<Vec<u8>, String> {
        let json_val = json!({
            "direction":        Self::serialize_direction(&input.direction),
            "steps_number":     input.steps_number,
            "step_duration_ms": json!(&input.step_duration.as_millis()),
        });
        let json_string = match serde_json::to_string(&json_val) {
            Ok(str_val) => str_val,
            Err(err) => return Err(err.to_string()),
        };
        Ok(json_string.into_bytes())
    }
}

pub struct JsonResponseParser;

impl JsonResponseParser {
    fn parse_result(json_data: &Value) -> Result<StepperMotorResponseCode, String> {
        let Some(result) = json_data.get("result") else {
            return Err("missing result field".to_string());
        };
        let Some(result) = result.as_i64() else {
            return Err("result field has wrong format".to_string());
        };
        match result {
            0 => Ok(StepperMotorResponseCode::OK),
            1 => Ok(StepperMotorResponseCode::ERROR),
            _ => Err(format!("unsupported result value: {}", result)),
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

    fn parse_state(json_data: &Value) -> Result<Option<StepperMotorState>, String> {
        let state_opt = json_data.get("state");
        if state_opt.is_none() {
            return Ok(None);
        }
        let Some(state) = state_opt.unwrap().as_i64() else {
            return Err("state field has wrong format".to_string());
        };
        match state {
            0 => Ok(Some(StepperMotorState::DISABLED)),
            1 => Ok(Some(StepperMotorState::ENABLED)),
            _ => Err(format!("unsupported state value: {}", state)),
        }
    }
}

impl DataTransformer<Vec<u8>, StepperMotorResponse, String> for JsonResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<StepperMotorResponse, String> {
        let json_val: Value = serde_json::from_slice(input).map_err(|err| err.to_string())?;
        Ok(StepperMotorResponse {
            code: Self::parse_result(&json_val)?,
            message: Self::parse_message(&json_val)?,
            state: Self::parse_state(&json_val)?,
        })
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn json_request_ser_sanity() {
        // GIVEN
        let test_request = StepperMotorRequest {
            step_duration: Duration::from_millis(1823),
            steps_number: 10,
            direction: StepperMotorDirection::CCW,
        };
        let expected_value = json!({
            "direction": JsonRequestSerializer::serialize_direction(&test_request.direction),
            "steps_number": test_request.steps_number,
            "step_duration_ms": test_request.step_duration.as_millis(),
        });

        // WHEN
        let request_serializer = JsonRequestSerializer;

        // THEN
        let serial_request = request_serializer.transform(&test_request).unwrap();
        let parsed_serial_request: Value = serde_json::from_slice(&serial_request).unwrap();
        assert_eq!(expected_value, parsed_serial_request);
    }

    #[test]
    fn json_response_par_sanity() {
        // GIVEN
        let succ_resp_val = json!({
            "result": 0,
        });
        let fail_msg = "the reason is ...";
        let fail_resp_val = json!({
            "result": 1,
            "what": fail_msg,
        });

        // WHEN
        let response_parser = JsonResponseParser;

        // THEN
        let request_serial_data = serde_json::to_string(&succ_resp_val).unwrap().into_bytes();
        let request_parsed = response_parser.transform(&request_serial_data).unwrap();
        assert_eq!(request_parsed.code, StepperMotorResponseCode::OK);
        let request_serial_data = serde_json::to_string(&fail_resp_val).unwrap().into_bytes();
        let request_parsed = response_parser.transform(&request_serial_data).unwrap();
        assert_eq!(request_parsed.code, StepperMotorResponseCode::ERROR);
    }
}