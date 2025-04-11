use std::time::Duration;

#[derive(Clone)]
pub struct StepperMotorRequest {
    pub motor_id: String,
    pub steps_number: usize,
    pub direction: StepperMotorDirection,
    pub step_duration: Duration,
}

#[derive(Clone)]
pub enum StepperMotorDirection {
    CCW,
    CW,
}

#[derive(Clone)]
pub enum StepperMotorResponse {
    SUCCESS,
    FAILURE(String),
}