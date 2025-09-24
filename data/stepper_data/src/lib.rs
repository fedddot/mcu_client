use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepperApiRequest {
    pub request_type: RequestType,
	pub direction: Option<Direction>,
    pub step_duration_us: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RequestType {
    Enable,
    Disable,
    Steps,
    Status
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Disabled,
    Enabled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepperApiResponse {
    pub result: ResultCode,
    pub message: Option<String>,
    pub status: Option<Status>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResultCode {
    Success,
    Failure,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_request() {
        let request = StepperApiRequest {
            request_type: RequestType::Steps,
            direction: Some(Direction::Clockwise),
            step_duration_us: Some(1000),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized Request: {serialized}");
    }
}
