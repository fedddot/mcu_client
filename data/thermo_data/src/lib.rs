#[derive(Clone, Debug)]
pub struct ThermostatApiRequest {
    pub request_type: RequestType,
	pub set_temperature: Option<f32>,
    pub time_resolution_ms: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RequestType {
    Start,
    Stop,
    SetTemperature,
}

#[derive(Clone, Debug)]
pub struct ThermostatApiResponse {
    pub status: StatusCode,
    pub message: Option<String>,
    pub current_temperature: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatusCode {
    Success,
    Failure,
}