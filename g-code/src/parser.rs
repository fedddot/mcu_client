use crate::{Command, GcodeData};

pub struct GcodeParser;

impl GcodeParser {
    pub fn parse(&self, gcode_line: &str) -> GcodeData {
        GcodeData {
            command: self.parse_cmd(gcode_line),
            target: todo!(),
            rotation_center: todo!(),
            speed: todo!(),
        }
    }

    fn parse_cmd(&self, gcode_line: &str) -> Command {
        todo!()
    }
}