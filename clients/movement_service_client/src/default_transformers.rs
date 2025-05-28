use serde_json::{json, Value};

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
        match api_request {
            MovementApiRequest::Config { .. } => Self::serialize_config_data(api_request),
            MovementApiRequest::LinearMovement { .. } => Self::serialize_linear_data(api_request),
            MovementApiRequest::RotationalMovement { .. } => Self::serialize_rotation_data(api_request),
        }
    }

    fn serialize_config_data(data: &MovementApiRequest) -> Value {
        let MovementApiRequest::Config { axes_configs } = data else {
            panic!("Expected Config variant");
        };
        let axis_to_string = |axis: &Axis| match axis {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        };
        let mut config_data = serde_json::Value::default();
        for (axis, config) in axes_configs {
            let axis_config = json!(
                {
                    "step_length": config.step_length,
                    "directions_mapping": config.directions_mapping,
                    "stepper_cfg": {
                        "enable_pin": config.stepper_config.enable_pin,
                        "step_pin": config.stepper_config.step_pin,
                        "dir_pin": config.stepper_config.dir_pin,
                        "hold_time_us": config.stepper_config.hold_time_us,
                    }
                }
            );
            config_data[axis_to_string(axis)] = axis_config;
        }
        config_data
    }

    fn serialize_linear_data(data: &MovementApiRequest) -> Value {
        let MovementApiRequest::LinearMovement { destination, speed } = data else {
            panic!("Expected LinearMovement variant");
        };
        json!(
            {
                "destination": Self::serialize_vector(destination),
                "speed": json!(speed),
            }
        )
    }

    fn serialize_vector(vector: &Vector<f32>) -> Value {
        json!(
            {
                "x": json!(vector.get(&Axis::X)),
                "y": json!(vector.get(&Axis::Y)),
                "z": json!(vector.get(&Axis::Z)),
            }
        )
    }

    fn serialize_rotation_data(_data: &MovementApiRequest) -> Value {
        todo!("serialize_rotation_data is not implemented yet")
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
        let json_val: Value = serde_json::from_slice(input).map_err(|err| err.to_string())?;
        Ok(MovementApiResponse {
            status: Self::parse_result(&json_val)?,
            message: Self::parse_message(&json_val)?,
        })
    }
}

#[cfg(test)]
mod test {
    use movement_data::Vector;

    use super::*;

    #[test]
    fn json_request_ser_sanity() {
        // GIVEN
        let test_request = MovementApiRequest::LinearMovement {
            destination: Vector::new(1.0, 2.0, 3.0),
            speed: 4.9,
        };
        let expected_value = json!({
            "type": JsonRequestSerializer::serialize_request_type(&test_request),
            "data": JsonRequestSerializer::serialize_request_data(&test_request),
        });

        // WHEN
        let request_serializer = JsonRequestSerializer;

        // THEN
        let serial_request = request_serializer.transform(&test_request).unwrap();
        let parsed_serial_request: Value = serde_json::from_slice(&serial_request).unwrap();
        assert_eq!(expected_value, parsed_serial_request);
    }
}