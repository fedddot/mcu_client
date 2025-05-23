use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum MovementApiRequest {
    Config {
        x_step_length: f32,
        y_step_length: f32,
        z_step_length: f32,
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

#[derive(Clone, Debug)]
pub struct MovementApiResponse {
    pub status: StatusCode,
    pub message: Option<String>,
}

#[derive(Clone, Debug)]
pub enum StatusCode {
    Success,
    Error,
}

#[derive(Clone, Debug)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}