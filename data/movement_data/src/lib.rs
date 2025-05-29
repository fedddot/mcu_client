use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MovementApiRequest {
    Config {
        axes_configs: HashMap<Axis, AxisConfig>,
    },
    LinearMovement {
        destination: Vector<f32>,
        speed: f32,
    },
    RotationalMovement {
        destination: Vector<f32>,
        rotation_center: Vector<f32>,
        angle: f32,
        speed: f32,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxisConfig {
    pub stepper_config: PicoStepperConfig,
    pub step_length: f32,
    pub directions_mapping: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PicoStepperConfig {
    pub enable_pin: u32,
    pub step_pin: u32,
    pub dir_pin: u32,
    pub hold_time_us: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovementApiResponse {
    pub status: StatusCode,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatusCode {
    Success,
    Error,
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "success" => StatusCode::Success,
            "error" => StatusCode::Error,
            _ => panic!("Unknown status code: {}", s),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vector<T: Clone> {
	values: HashMap<Axis, T>,
}

impl<T: Clone> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        let mut values = HashMap::new();
        values.insert(Axis::X, x);
        values.insert(Axis::Y, y);
        values.insert(Axis::Z, z);
        Self {
            values
        }
    }
    pub fn get(&self, axis: &Axis) -> &T {
		self.values.get(axis).unwrap()
	}
    pub fn set(&mut self, axis: &Axis, val: T) {
		self.values.insert(axis.clone(), val);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl TryFrom<&str> for Axis {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            "z" => Ok(Axis::Z),
            _ => Err(format!("unsupported axis: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_api_request_serialization() {
        // GIVEN
        let test_destination = Vector::new(1.0, 2.0, 3.0);
        let test_speed = 4.0;
        let request = MovementApiRequest::LinearMovement {
            destination: test_destination.clone(),
            speed: test_speed,
        };

        // THEN
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: MovementApiRequest = serde_json::from_str(&serialized).unwrap();
        println!("serialized data:\n{}", serialized);
        let MovementApiRequest::LinearMovement { destination, speed } = deserialized else {
            panic!("Deserialized request is not LinearMovement");
        };
        assert_eq!(test_speed, speed);
        [Axis::X, Axis::Y, Axis::Z]
            .iter()
            .for_each(|axis| assert_eq!(test_destination.get(axis), destination.get(axis)));
    }
}