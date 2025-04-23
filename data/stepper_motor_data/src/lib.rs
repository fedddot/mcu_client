use std::time::Duration;

#[derive(Clone, Debug)]
pub struct StepperMotorRequest {
    pub motor_id: String,
    pub direction: StepperMotorDirection,
    pub steps_number: usize,
    pub step_duration: Duration,
}

#[derive(Clone, Debug)]
pub enum StepperMotorDirection {
    CCW = 0,
    CW = 1,
}

#[derive(Clone, Debug)]
pub struct StepperMotorResponse {
    pub code: StepperMotorResponseCode,
    pub message: Option<String>,
    pub state: Option<StepperMotorState>,
}

impl Default for StepperMotorResponse {
    fn default() -> Self {
        StepperMotorResponse {
            code: StepperMotorResponseCode::Exception,
            message: Some("default response".to_string()),
            state: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StepperMotorResponseCode {
    Ok,
    NotFound,
    Unsupported,
    BadRequest,
    Exception,
}

#[derive(Clone, Debug)]
pub enum StepperMotorState {
    DISABLED = 0,
    ENABLED = 1,
}