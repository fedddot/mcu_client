use movement_data::{Axis, Vector};

pub fn add_vectors(one: &Vector<f32>, other: &Vector<f32>) -> Vector<f32> {
    let mut result = Vector::new(0.0, 0.0, 0.0);
    [Axis::X, Axis::Y, Axis::Z]
        .iter()
        .for_each(|axis| result.set(axis, one.get(axis) + other.get(axis)));
    result
}

pub fn sub_vectors(one: &Vector<f32>, other: &Vector<f32>) -> Vector<f32> {
    let mut result = Vector::new(0.0, 0.0, 0.0);
    [Axis::X, Axis::Y, Axis::Z]
        .iter()
        .for_each(|axis| result.set(axis, one.get(axis) - other.get(axis)));
    result
}