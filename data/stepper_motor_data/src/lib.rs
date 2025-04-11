use std::time::Duration;

pub struct StepperMotorRequest {
    pub motor_id: String,
    pub steps_number: usize,
    pub direction: StepperMotorDirection,
    pub step_duration: Duration,
}

pub enum StepperMotorDirection {
    CCW,
    CW,
}

pub enum StepperMotorResponse {
    SUCCESS,
    FAILURE(String),
}