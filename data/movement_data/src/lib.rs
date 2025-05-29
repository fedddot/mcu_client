use std::collections::HashMap;
use serde::ser::SerializeStruct;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MovementApiRequest {
    Config(AxesConfig),
    LinearMovement(LinearMovement),
    RotationalMovement(RotationalMovement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AxesConfig {
    pub x_axis_config: AxisConfig,
    pub y_axis_config: AxisConfig,
    pub z_axis_config: AxisConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinearMovement {
    pub destination: Vector<f32>,
    pub speed: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotationalMovement {
    destination: Vector<f32>,
    rotation_center: Vector<f32>,
    angle: f32,
    speed: f32,
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

#[derive(Clone, Debug)]
pub struct Vector<T: Clone> {
	values: HashMap<Axis, T>,
}

impl serde::Serialize for Vector<f32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Vector", 3)?;
        state.serialize_field("x", self.get(&Axis::X))?;
        state.serialize_field("y", self.get(&Axis::Y))?;
        state.serialize_field("z", self.get(&Axis::Z))?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for Vector<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut values = HashMap::new();
        let map: HashMap<String, f32> = HashMap::deserialize(deserializer)?;
        for (key, value) in map {
            match Axis::try_from(key.as_str()) {
                Ok(axis) => { values.insert(axis, value); },
                Err(e) => return Err(serde::de::Error::custom(e)),
            }
        }
        Ok(Self { values })
    }
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
        let linear_movement = LinearMovement {
            destination: Vector::new(1.0, 2.0, 3.0),
            speed: 4.0,
        };
        let request = MovementApiRequest::LinearMovement(linear_movement.clone());

        // THEN
        let serialized = serde_json::to_string(&linear_movement).unwrap();
        println!("serialized linear movement:\n{}", serialized);
        let deserialized: LinearMovement = serde_json::from_str(&serialized).unwrap();
        assert_eq!(linear_movement.speed, deserialized.speed);
        [Axis::X, Axis::Y, Axis::Z]
            .iter()
            .for_each(
                |axis| assert_eq!(
                    linear_movement.destination.get(axis),
                    deserialized.destination.get(axis)
                )
            );


        let serialized = serde_json::to_string(&request).unwrap();
        println!("serialized data:\n{}", serialized);
        let deserialized: MovementApiRequest = serde_json::from_str(&serialized).unwrap();
        let MovementApiRequest::LinearMovement(deserialized_movement) = deserialized else {
            panic!("Deserialized request is not LinearMovement");
        };
        assert_eq!(linear_movement.speed, deserialized_movement.speed);
        
    }

    #[test]
    fn test_movement_api_response_serialization() {
        // GIVEN
        let response = MovementApiResponse {
            status: StatusCode::Success,
            message: Some("movement completed successfully".to_string()),
        };

        // THEN
        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: MovementApiResponse = serde_json::from_str(&serialized).unwrap();
        println!("serialized data:\n{}", serialized);
        assert_eq!(response.status, deserialized.status);
        assert_eq!(response.message, deserialized.message);
    }
}