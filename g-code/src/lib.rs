pub use movement_data::Vector;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    G00, // Rapid Position
    G01, // Linear Movement
}

#[derive(Clone, Debug)]
pub struct GcodeData {
    pub command: Command,
    pub target: Option<Vector<f32>>,
    pub rotation_center: Option<Vector<f32>>,
    pub speed: Option<f32>,
}

mod parser;