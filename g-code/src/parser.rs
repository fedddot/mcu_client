use std::collections::HashMap;

use movement_data::{Axis, Vector};

use crate::{Command, GcodeData};

#[derive(Default)]
pub struct GcodeParser;

impl GcodeParser {
    pub fn parse(&self, gcode_line: &str) -> Result<GcodeData, String> {
        let command = Self::parse_cmd(gcode_line)?;
        let target = Self::parse_target(gcode_line)?;
        let rotation_center = Self::parse_rotation_center(gcode_line)?;
        let speed = Self::parse_speed(gcode_line)?;
        Ok(GcodeData { command, target, rotation_center, speed })
    }

    fn parse_cmd(gcode_line: &str) -> Result<Command, String> {
        let Some(command_str) = gcode_line.split_whitespace().next() else {
            return Err("received gcode line doesn't contain g-command".to_string());
        };
        match command_str {
            "G0" | "G00" => Ok(Command::G00),
            "G1" | "G01" => Ok(Command::G01),
            any_other => Err(format!("unsupported G-command received: {any_other}")),
        }
    }

    fn parse_vector(gcode_line: &str, axes_mapping: &HashMap<Axis, char>) -> Result<Option<Vector<f32>>, String> {
        let mut vector = Vector::new(0.0, 0.0, 0.0);
        let mut found_any = false;
        for (axis, tag) in axes_mapping.iter() {
            for token in gcode_line.split_whitespace() {
                if let Some(value) = token.strip_prefix(*tag) {
                    let Ok(value) = value.parse::<f32>() else {
                        return Err(format!("invalid {tag} value: {value}"));
                    };
                    vector.set(axis, value);
                    found_any = true;
                }
            }
        }
        match found_any {
            true => Ok(Some(vector)),
            false => Ok(None),
        }
    }

    fn parse_target(gcode_line: &str) -> Result<Option<Vector<f32>>, String> {
        Self::parse_vector(
            gcode_line,
            &HashMap::from([
                (Axis::X, 'X'),
                (Axis::Y, 'Y'),
                (Axis::Z, 'Z'),
            ]),
        )
    }

    fn parse_rotation_center(gcode_line: &str) -> Result<Option<Vector<f32>>, String> {
        Self::parse_vector(
            gcode_line,
            &HashMap::from([
                (Axis::X, 'I'),
                (Axis::Y, 'J'),
                (Axis::Z, 'K'),
            ]),
        )
    }

    fn parse_speed(gcode_line: &str) -> Result<Option<f32>, String> {
        let speed_tag = 'F';
        for token in gcode_line.split_whitespace() {
            if let Some(value) = token.strip_prefix(speed_tag) {
                let Ok(speed_val) = value.parse::<f32>() else {
                    return Err(format!("invalid speed value: {value}"));
                };
                return Ok(Some(speed_val));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        // GIVEN
        let target_vector = Vector::<f32>::new(1.0, 2.0, 0.0);
        let rotation_vector = Vector::<f32>::new(0.0, 6.7, 0.0);
        let speed: f32 = 10.1;
        let test_gcode_str = format!(
            "G0 X{} Y{} J{} F{}",
            target_vector.get(&Axis::X),
            target_vector.get(&Axis::Y),
            rotation_vector.get(&Axis::Y),
            speed,
        );

        // WHEN
        let instance = GcodeParser;

        // THEN
        let result = instance.parse(&test_gcode_str);
        assert!(result.is_ok());
        let result = result.unwrap();
        println!("result: {result:?}");
        assert_eq!(Command::G00, result.command);
    }
}