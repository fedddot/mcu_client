pub use vector::{Axis, Vector};
pub use movement_data_types::{LinearMovementData, RotationalMovementData};

#[derive(Clone, Debug)]
pub struct MovementManagerRequest {
	pub movement_type: MovementType,
}

#[derive(Clone, Debug)]
pub enum MovementType {
    Linear(LinearMovementData),
    Rotational(RotationalMovementData),
}

pub struct MovementManagerResponse {
    pub code: ResultCode,
	pub message: Option<String>,
}

#[derive(Clone)]
pub enum ResultCode {
    Ok = 0,
    BadRequest = 1,
    Exception = 2,
}

mod movement_data_types;
mod vector;