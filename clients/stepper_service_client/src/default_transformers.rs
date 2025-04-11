use serde_json::{json, Value};

use stepper_motor_data::{StepperMotorDirection, StepperMotorRequest, StepperMotorResponse};
use crate::DataTransformer;

pub struct JsonRequestSerializer;

impl JsonRequestSerializer {
    fn serialize_direction(direction: &StepperMotorDirection) -> Value {
        match direction {
            StepperMotorDirection::CCW => json!("CCW"),
            StepperMotorDirection::CW => json!("CW"),
        }
    }
}

impl DataTransformer<StepperMotorRequest, Vec<u8>, String> for JsonRequestSerializer {
    fn transform(&self, input: &StepperMotorRequest) -> Result<Vec<u8>, String> {
        let json_val = json!({
            "motor_id":         input.motor_id,
            "steps_number":     input.steps_number,
            "direction":        Self::serialize_direction(&input.direction),
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

impl DataTransformer<Vec<u8>, StepperMotorResponse, String> for JsonResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<StepperMotorResponse, String> {
        let json_val: Value = serde_json::from_slice(input).map_err(|err| err.to_string())?;
        let Some(result_val) = json_val.get("result") else {
            return Err("missing result field in the report data".to_string());
        };
        let Some(result_str) = result_val.as_str() else {
            return Err("result field has wrong format".to_string());
        };
        match result_str {
            "SUCCESS" => Ok(StepperMotorResponse::SUCCESS),
            "FAILURE" => {
                let message = match json_val.get("what") {
                    Some(msg) => msg.to_string(),
                    _ => "".to_string(),
                };
                Ok(StepperMotorResponse::FAILURE(message))
            },
            any_else => Err(format!("failed to parse response: unsupported result value: {}", any_else)),
        }
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
            motor_id: "test_motor".to_string(),
            steps_number: 10,
            direction: StepperMotorDirection::CCW,
            step_duration: Duration::from_millis(1823),
        };
        let expected_value = json!({
            "motor_id": test_request.motor_id,
            "steps_number": test_request.steps_number,
            "direction": JsonRequestSerializer::serialize_direction(&test_request.direction),
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
            "result": "SUCCESS",
        });
        let fail_msg = "the reason is ...";
        let fail_resp_val = json!({
            "result": "FAILURE",
            "what": fail_msg,
        });

        // WHEN
        let response_parser = JsonResponseParser;

        // THEN
        let request_serial_data = serde_json::to_string(&succ_resp_val).unwrap().into_bytes();
        let request_parsed = response_parser.transform(&request_serial_data).unwrap();
        assert!(matches!(request_parsed, StepperMotorResponse::SUCCESS));
        let request_serial_data = serde_json::to_string(&fail_resp_val).unwrap().into_bytes();
        let request_parsed = response_parser.transform(&request_serial_data).unwrap();
        assert!(matches!(request_parsed, StepperMotorResponse::FAILURE(_)));
    }
}