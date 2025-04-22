use std::time::Duration;

#[derive(Clone, Debug)]
pub struct StepperMotorRequest {
    pub direction: StepperMotorDirection,
    pub steps_number: usize,
    pub step_duration: Duration,
}

#[derive(Clone, Debug)]
pub enum StepperMotorDirection {
    CCW = 0,
    CW = 1,
}

#[derive(Clone)]
pub struct StepperMotorResponse {
    pub code: StepperMotorResponseCode,
    pub message: Option<String>,
    pub state: Option<StepperMotorState>,
}

impl Default for StepperMotorResponse {
    fn default() -> Self {
        StepperMotorResponse {
            code: StepperMotorResponseCode::ERROR,
            message: Some("default response".to_string()),
            state: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StepperMotorResponseCode {
    OK = 0,
    ERROR = 1,
}

#[derive(Clone)]
pub enum StepperMotorState {
    DISABLED = 0,
    ENABLED = 1,
}