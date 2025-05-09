use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct LinearMovementData {
    pub destination: Vector<f32>,
    pub speed: f32,
}

#[derive(Clone, Debug)]
pub struct RotationalMovementData;