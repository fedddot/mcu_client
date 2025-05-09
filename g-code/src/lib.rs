#[derive(Default)]
pub struct GcodeProcessor {
    parser: GcodeParser,
}

impl GcodeProcessor {
    pub fn new() -> Self {
        GcodeProcessor::default()
    }

    pub fn process(&self, gcode_line: &str) -> Result<(), String> {
        let _ = self.parser.parse(gcode_line)?;
        Err("not implemented".to_string())
    }
}

mod parser;

use movement_data::Vector;
use parser::GcodeParser;

#[derive(Clone, Debug, PartialEq)]
enum Command {
    G00, // Rapid Position
    G01, // Linear Movement
}

#[derive(Clone, Debug)]
struct GcodeData {
    pub command: Command,
    pub target: Option<Vector<f32>>,
    pub rotation_center: Option<Vector<f32>>,
    pub speed: Option<f32>,
}
