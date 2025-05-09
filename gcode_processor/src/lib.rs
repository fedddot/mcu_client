type MovementServiceClient = dyn ServiceClient<MovementManagerRequest, MovementManagerResponse, String>;

pub struct GcodeProcessor {
    parser: GcodeParser,
    fast_movement_speed: f32,
    movement_service_client: Box<MovementServiceClient>,
}

impl GcodeProcessor {
    pub fn new(
        fast_movement_speed: f32,
        movement_service_client: Box<MovementServiceClient>,
    ) -> Self {
        Self {
            parser: GcodeParser,
            fast_movement_speed,
            movement_service_client,
        }
    }

    pub fn process(&mut self, gcode_line: &str) -> Result<(), String> {
        let gcode_data = self.parser.parse(gcode_line)?;
        let movement_request = self.generate_movement_request(&gcode_data)?;
        let movement_response = self.movement_service_client.run_request(&movement_request)?;
        match movement_response.code {
            ResultCode::Ok => Ok(()),
            _ => {
                let mut error_msg = "a failure response received from the movement service".to_string();
                if let Some(what) = movement_response.message {
                    error_msg = format!("{error_msg}, what: {what}");
                }
                Err(error_msg)
            }
        }
    }

    fn generate_movement_request(&self, gcode_data: &GcodeData) -> Result<MovementManagerRequest, String> {
        match &gcode_data.command {
            Command::G00 => {
                let Some(destination) = &gcode_data.target else {
                    return Err("G00 gcode data must have target vector".to_string());
                };
                let movement_data = LinearMovementData {
                    destination: destination.clone(),
                    speed: self.fast_movement_speed,
                };
                Ok(MovementManagerRequest { movement_type: MovementType::Linear(movement_data) })
            },
            any_other => Err(format!("unsupported command received: {any_other:?}")),
        }
    }
}

mod parser;

use client::ServiceClient;
use movement_data::{LinearMovementData, MovementManagerRequest, MovementManagerResponse, MovementType, ResultCode, Vector};
use parser::GcodeParser;

#[derive(Clone, Debug, PartialEq)]
enum Command {
    G00, // Rapid Position
    G01, // Linear Movement
}

#[derive(Clone, Debug)]
struct GcodeData {
    pub command: Command,
    pub target: Option<Vector<f32>>,
    pub rotation_center: Option<Vector<f32>>,
    pub speed: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    #[test]
    fn sanity() {
        // GIVEN
        let gcode_line = "G00 X1.2 Y3.4 Z5.6";
        let mut mock_service_client = MockServiceClient::default();
        let fast_speed = 7.8;

        // WHEN
        mock_service_client
            .expect_run_request()
            .returning(|_| Ok(MovementManagerResponse { code: ResultCode::Ok, message: None }));
        let mut instance = GcodeProcessor::new(fast_speed, Box::new(mock_service_client));

        // THEN
        let result = instance.process(gcode_line);
        assert!(result.is_ok());
    }

    mock! {
        pub ServiceClient {}

        impl ServiceClient<MovementManagerRequest, MovementManagerResponse, String> for ServiceClient {
            fn run_request(&mut self, request: &MovementManagerRequest) -> Result<MovementManagerResponse, String>;
        }
    }
}