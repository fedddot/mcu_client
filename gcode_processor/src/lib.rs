use std::collections::HashMap;

use client::ServiceClient;
use movement_data::{
    Axis, AxisConfig, MovementApiRequest, MovementApiResponse, StatusCode, Vector
};

pub type MovementServiceClient = dyn ServiceClient<MovementApiRequest, MovementApiResponse, String>;

pub struct GcodeProcessor {
    parser: parser::GcodeParser,
    fast_movement_speed: f32,
    default_movement_speed: f32,
    movement_service_client: Box<MovementServiceClient>,
    state_storage: Box<dyn StateStorage>,
}

impl GcodeProcessor {
    pub fn new(
        fast_movement_speed: f32,
        default_movement_speed: f32,
        movement_service_client: Box<MovementServiceClient>,
        axes_configs: &HashMap<Axis, AxisConfig>,
        state_storage: Box<dyn StateStorage>,
    ) -> Self {
        let mut instance = Self {
            parser: parser::GcodeParser,
            fast_movement_speed,
            default_movement_speed,
            movement_service_client,
            state_storage,
        };
        let config_request = MovementApiRequest::Config { axes_configs: axes_configs.clone() };
        let config_response = instance
            .movement_service_client
            .run_request(&config_request)
            .expect("failed to run configuring request to the movement service");
        if config_response.status != StatusCode::Success {
            panic!("configuration request failed: {:?}", config_response.message);
        }
        instance
    }

    pub fn process(&mut self, gcode_line: &str) -> Result<(), String> {
        let gcode_data = self.parser.parse(gcode_line)?;
        match gcode_data.command {
            Command::G90 | Command::G91 => self.process_control_command(&gcode_data),
            _ => self.process_movement_command(&gcode_data),
        }
    }

    pub fn state(&self) -> GcodeProcessorState {
        self.state_storage.read_state().unwrap()
    }

    fn process_movement_command(&mut self, gcode_data: &GcodeData) -> Result<(), String> {
        let movement_request = self.generate_movement_request(gcode_data)?;
        let movement_response = self.movement_service_client.run_request(&movement_request)?;
        match movement_response.status {
            StatusCode::Success => {
                let state = self.state_storage.read_state()?;
                let state = Self::apply_movement_to_state(&state, &movement_request);
                self.state_storage.write_state(&state)?;
                Ok(())
            },
            _ => {
                let mut error_msg = "a failure response received from the movement service".to_string();
                if let Some(what) = movement_response.message {
                    error_msg = format!("{error_msg}, what: {what}");
                }
                Err(error_msg)
            }
        }
    }

    fn process_control_command(&mut self, gcode_data: &GcodeData) -> Result<(), String> {
        match &gcode_data.command {
            Command::G90 => {
                let mut state = self.state_storage.read_state()?;
                state.coordinates_type = CoordinatesType::Absolute;
                self.state_storage.write_state(&state)?;
                Ok(())
            },
            Command::G91 => {
                let mut state = self.state_storage.read_state()?;
                state.coordinates_type = CoordinatesType::Relative;
                self.state_storage.write_state(&state)?;
                Ok(())
            },
            any_other => Err(format!("unsupported control command received: {any_other:?}")),
        }
    }

    fn apply_movement_to_state(state: &GcodeProcessorState, movement_request: &MovementApiRequest) -> GcodeProcessorState {
        let mut state = state.clone();
        let movement_vector = match movement_request {
            MovementApiRequest::LinearMovement { destination, speed: _ } => destination,
            _ => panic!("unsupported movement type"),
        };
        state.current_position = vector_operations::add_vectors(&state.current_position, movement_vector);
        state
    }

    fn apply_state_to_target_vector(target_vector: &[VectorCoordinateToken], state: &GcodeProcessorState) -> Vector<f32> {
        let mut target = Vector::default();
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            let token_projection_opt = target_vector
                .iter()
                .find(|(token_axis, _)| *token_axis == axis)
                .map(|(_, projection)| *projection);
            match state.coordinates_type {
                CoordinatesType::Absolute => {
                    let token_projection = token_projection_opt
                        .unwrap_or(*state.current_position.get(&axis));
                    target.set(&axis, token_projection - state.current_position.get(&axis));
                },
                CoordinatesType::Relative => {
                    let token_projection = token_projection_opt
                        .unwrap_or(0.0);
                    target.set(&axis, token_projection);
                },
            }
        }
        target
    }

    fn generate_movement_request(&self, gcode_data: &GcodeData) -> Result<MovementApiRequest, String> {
        match &gcode_data.command {
            Command::G00 => {
                let state = self.state_storage.read_state()?;
                Ok(MovementApiRequest::LinearMovement {
                    destination: Self::apply_state_to_target_vector(&gcode_data.target_tokens, &state),
                    speed: self.fast_movement_speed,
                })
            },
            Command::G01 => {
                let speed = match gcode_data.speed {
                    Some(speed_data) => speed_data,
                    _ => self.default_movement_speed,
                };
                let state = self.state_storage.read_state()?;
                Ok(MovementApiRequest::LinearMovement {
                    destination: Self::apply_state_to_target_vector(&gcode_data.target_tokens, &state),
                    speed,
                })
            },
            any_other => Err(format!("unsupported movement command received: {any_other:?}")),
        }
    }
}

pub trait StateStorage {
    fn read_state(&self) -> Result<GcodeProcessorState, String>;
    fn write_state(&mut self, state: &GcodeProcessorState) -> Result<(), String>;
}

#[derive(Clone)]
pub struct GcodeProcessorState {
    pub current_position: Vector<f32>,
    pub coordinates_type: CoordinatesType,
}

impl Default for GcodeProcessorState {
    fn default() -> Self {
        Self {
            current_position: Vector::new(0.0, 0.0, 0.0),
            coordinates_type: CoordinatesType::Absolute,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoordinatesType {
    Relative,
    Absolute,
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    G00, // Rapid Position
    G01, // Linear Movement
    G90, // Absolute Mode
    G91, // Relative Mode
    _G03,
}

#[derive(Clone, Debug)]
struct GcodeData {
    pub command: Command,
    pub target_tokens: Vec<VectorCoordinateToken>,
    pub _rotation_center_tokens: Vec<VectorCoordinateToken>,
    pub speed: Option<f32>,
}

type VectorCoordinateToken = (Axis, f32);

mod parser;
mod vector_operations;

#[cfg(test)]
mod tests;