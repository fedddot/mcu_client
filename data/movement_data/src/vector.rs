use std::collections::HashMap;

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
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}