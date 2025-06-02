use movement_data::{Axis, Vector};

pub fn add_vectors(left: &Vector<f32>, right: &Vector<f32>) -> Vector<f32> {
    let mut result = Vector::new(0.0, 0.0, 0.0);
    [Axis::X, Axis::Y, Axis::Z]
        .iter()
        .for_each(|axis| result.set(axis, left.get(axis) + right.get(axis)));
    result
}

pub fn _sub_vectors(left: &Vector<f32>, right: &Vector<f32>) -> Vector<f32> {
    let mut result = Vector::new(0.0, 0.0, 0.0);
    [Axis::X, Axis::Y, Axis::Z]
        .iter()
        .for_each(|axis| result.set(axis, left.get(axis) - right.get(axis)));
    result
}