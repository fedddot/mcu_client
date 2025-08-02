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