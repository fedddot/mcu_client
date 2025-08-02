use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThermostatApiRequest {
    pub request_type: RequestType,
	pub set_temperature: Option<f32>,
    pub time_resolution_ms: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RequestType {
    Start,
    Stop,
    GetTemperature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThermostatApiResponse {
    pub status: StatusCode,
    pub message: Option<String>,
    pub current_temperature: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatusCode {
    Success,
    Failure,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_request() {
        let request = ThermostatApiRequest {
            request_type: RequestType::Start,
            set_temperature: Some(22.5),
            time_resolution_ms: Some(1000),
        };
        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized Request: {serialized}");
    }
}
