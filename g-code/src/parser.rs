use std::collections::HashMap;

use movement_data::{Axis, Vector};

use crate::{Command, GcodeData};

pub struct GcodeParser;

impl GcodeParser {
    pub fn parse(&self, gcode_line: &str) -> GcodeData {
        GcodeData {
            command: self.parse_cmd(gcode_line),
            target: self.parse_target(gcode_line),
            rotation_center: self.parse_rotation_center(gcode_line),
            speed: self.parse_speed(gcode_line),
        }
    }

    fn parse_cmd(&self, gcode_line: &str) -> Command {
        let command_str = gcode_line
            .split_whitespace()
            .next()
            .expect("G-code line is empty");
        match command_str {
            "G0" | "G00" => Command::G00,
            "G1" | "G01" => Command::G01,
            any_other => panic!("unsupported G-command received: {any_other}"),
        }
    }

    fn parse_target(&self, gcode_line: &str) -> Option<Vector<f32>> {
        let mut target = Vector::new(0.0, 0.0, 0.0);
        let mut found_any = false;
        let axis_to_tag_mapping = HashMap::from([
            (Axis::X, 'X'),
            (Axis::Y, 'Y'),
            (Axis::Z, 'Z'),
        ]);
        for token in gcode_line.split_whitespace() {
            axis_to_tag_mapping
                .iter()
                .for_each(
                    |(axis, tag)| {
                        if let Some(value) = token.strip_prefix(*tag) {
                            target.set(axis, value.parse::<f32>().unwrap_or_else(|_| panic!("Invalid {tag} value: {value}")));
                            found_any = true;
                        }
                    }
                );
        }
        match found_any {
            true => Some(target),
            false => None,
        }
    }

    fn parse_rotation_center(&self, gcode_line: &str) -> Option<Vector<f32>> {
        let mut rotation_center = Vector::new(0.0, 0.0, 0.0);
        let mut found_any = false;
        let axis_to_tag_mapping = HashMap::from([
            (Axis::X, 'I'),
            (Axis::Y, 'J'),
            (Axis::Z, 'K'),
        ]);
        for token in gcode_line.split_whitespace() {
            axis_to_tag_mapping
                .iter()
                .for_each(
                    |(axis, tag)| {
                        if let Some(value) = token.strip_prefix(*tag) {
                            rotation_center.set(axis, value.parse::<f32>().unwrap_or_else(|_| panic!("invalid {tag} value: {value}")));
                            found_any = true;
                        }
                    }
                );
        }
        match found_any {
            true => Some(rotation_center),
            false => None,
        }
    }

    fn parse_speed(&self, gcode_line: &str) -> Option<f32> {
        let speed_tag = 'F';
        for token in gcode_line.split_whitespace() {
            if let Some(value) = token.strip_prefix(speed_tag) {
                let speed_val = value.parse::<f32>().unwrap_or_else(|_| panic!("invalid speed value: {value}"));
                return Some(speed_val);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let test_gcode_str = "G0 X10.0 Y20.0 J1 F40.1";

        // WHEN
        let instance = GcodeParser;

        // THEN
        let result = instance.parse(test_gcode_str);
        assert_eq!(Command::G00, result.command);
        println!("result: {result:?}");
    }
}