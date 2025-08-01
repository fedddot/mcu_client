use std::collections::HashMap;

use movement_data::{Axis, MovementApiRequest, MovementApiResponse, StatusCode};
use prost::Message;

use crate::DataTransformer;

fn serialize_movement_request(request: &MovementApiRequest) -> Vec<u8> {
    match request {
        MovementApiRequest::LinearMovement { destination, speed } => {
            let target = pb::Vector {
                x: *destination.get(&Axis::X),
                y: *destination.get(&Axis::Y),
                z: *destination.get(&Axis::Z),
            };
            let pb_request = pb::MovementApiRequest {
                request: Some(pb::movement_api_request::Request::LinearMovementRequest(
                    pb::LinearMovementRequest {
                        speed: *speed,
                        target: Some(target),
                    },
                )),
            };
            pb_request.encode_to_vec()
        },
        MovementApiRequest::RotationalMovement { destination, rotation_center, angle, speed } => {
            let target = pb::Vector {
                x: *destination.get(&Axis::X),
                y: *destination.get(&Axis::Y),
                z: *destination.get(&Axis::Z),
            };
            let rotation_center = pb::Vector {
                x: *rotation_center.get(&Axis::X),
                y: *rotation_center.get(&Axis::Y),
                z: *rotation_center.get(&Axis::Z),
            };
            let pb_request = pb::MovementApiRequest {
                request: Some(pb::movement_api_request::Request::RotationMovementRequest(
                    pb::RotationMovementRequest {
                        speed: *speed,
                        target: Some(target),
                        rotation_center: Some(rotation_center),
                        angle: *angle,
                    },
                )),
            };
            pb_request.encode_to_vec()
        },
        MovementApiRequest::Config { axes_configs } => {
            let x_cfg = axis_cfg_to_pb(axes_configs.get(&Axis::X).expect("X axis config missing"));
            let y_cfg = axis_cfg_to_pb(axes_configs.get(&Axis::Y).expect("Y axis config missing"));
            let z_cfg = axis_cfg_to_pb(axes_configs.get(&Axis::Z).expect("Z axis config missing"));
            let axes_config = pb::AxesConfig {
                x_axis_cfg: Some(x_cfg),
                y_axis_cfg: Some(y_cfg),
                z_axis_cfg: Some(z_cfg),
            };
            let pb_request = pb::MovementApiRequest {
                request: Some(
                    pb::movement_api_request::Request::ConfigRequest(
                        pb::ConfigRequest {
                            axes_config: Some(axes_config),
                        }
                    )
                ),
            };
            pb_request.encode_to_vec()
        },
    }
}

fn axis_cfg_to_pb(axis_cfg: &movement_data::AxisConfig) -> pb::AxisConfig {
    pb::AxisConfig {
        stepper_config: Some(pb::PicoStepperConfig {
            enable_pin: axis_cfg.stepper_config.enable_pin,
            step_pin: axis_cfg.stepper_config.step_pin,
            dir_pin: axis_cfg.stepper_config.dir_pin,
            hold_time_us: axis_cfg.stepper_config.hold_time_us,
        }),
        step_length: axis_cfg.step_length,
        directions_mapping: Some(directions_mapping_to_pb(&axis_cfg.directions_mapping)),
    }
}

fn directions_mapping_to_pb(directions_mapping: &HashMap<String, String>,
) -> pb::DirectionsMapping {
    let str_to_stepper_dir = |s: &str| match s {
        "CW" => pb::StepperDirection::Cw as i32,
        "CCW" => pb::StepperDirection::Ccw as i32,
        _ => panic!("unsupported stepper direction: {}", s),
    };
    pb::DirectionsMapping {
        positive: str_to_stepper_dir(directions_mapping
            .get("POSITIVE")
            .expect("missing POSITIVE direction")
        ),
        negative: str_to_stepper_dir(directions_mapping
            .get("NEGATIVE")
            .expect("missing NEGATIVE direction")
        ),
    }
}

fn parse_movement_response(data: &[u8]) -> Result<MovementApiResponse, String> {
    let pb_response = pb::MovementApiResponse::decode(data)
        .map_err(|e| format!("failed to decode response: {}", e))?;
    let pb_status = pb::StatusCode::try_from(pb_response.status)
        .map_err(|e| format!("failed to convert status code: {}", e))?;
    let status = match pb_status {
        pb::StatusCode::Success => StatusCode::Success,
        pb::StatusCode::Failure => StatusCode::Error,
    };
    Ok(MovementApiResponse {
        status,
        message: pb_response.message.into(),
    })
}

pub struct ProtoRequestSerializer;

impl DataTransformer<MovementApiRequest, Vec<u8>, String> for ProtoRequestSerializer {
    fn transform(&self, input: &MovementApiRequest) -> Result<Vec<u8>, String> {
        Ok(serialize_movement_request(input))
    }
}

pub struct ProtoResponseParser;

impl DataTransformer<Vec<u8>, MovementApiResponse, String> for ProtoResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<MovementApiResponse, String> {
        parse_movement_response(input)
    }
}

mod pb {
    tonic::include_proto!("movement_vendor_api");
}

#[cfg(test)]
mod tests {
    use movement_data::Vector;

    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let test_request = MovementApiRequest::LinearMovement {
            destination: Vector::new(0.1, 0.2, 0.3),
            speed: 1.0,
        };

        // THEN
        let serialized = serialize_movement_request(&test_request);
        println!("Serialized request: {:?}", serialized);
    }
}
