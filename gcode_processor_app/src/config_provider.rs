use std::collections::HashMap;

use movement_data::{Axis, AxisConfig};
use serde_json::Value;

pub struct ConfigProvider {
    config_data: Value,
}

impl ConfigProvider {
    pub fn new(config_path: &str) -> Result<Self, String> {
        let config_data = std::fs::read_to_string(config_path)
            .map_err(|e| format!("failed to read config file: {}", e))?;
        let config_data: Value = serde_json::from_str(&config_data)
            .map_err(|e| format!("failed to parse config json: {}", e))?;
        Ok(Self { config_data })
    }

    pub fn axes_config(&self) -> Result<HashMap<Axis, AxisConfig>, String> {
        let axes_config = self.config_data.get("axes_config")
            .ok_or("axes configuration not found in config")?
            .as_object()
            .ok_or("axes configuration is not an object")?;

        for (axis_str, axis_cfg) in axes_config.iter() {
            if !axis_cfg.is_object() {
                return Err(format!("axis config for '{}' is not an object", axis_str));
            }
        }
        let mut axes_map = HashMap::new();
        for (axis_str, axis_cfg) in axes_config {
            let axis = Axis::try_from(axis_str.as_str())?;
            let axis_config = Self::parse_axis_config(axis_cfg)?;
            axes_map.insert(axis, axis_config);
        }
        Ok(axes_map)
    }

    fn parse_axis_config(axis_cfg: &Value) -> Result<AxisConfig, String> {
        let stepper_config = axis_cfg.get("stepper_config")
            .ok_or("stepper configuration not found")?
            .as_object()
            .ok_or("stepper configuration is not an object")?;
        
        let step_length = axis_cfg.get("step_length")
            .and_then(|v| v.as_f64())
            .ok_or("step length is not a valid number")? as f32;

        let hold_time_us = axis_cfg.get("hold_time_us")
            .and_then(|v| v.as_u64())
            .ok_or("hold time is not a valid number")? as u32;

        let directions_mapping = axis_cfg.get("directions_mapping")
            .and_then(|v| v.as_object())
            .ok_or("directions mapping is not an object")?
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect();

        Ok(AxisConfig {
            stepper_config: PicoStepperConfig {
                enable_pin: stepper_config.get("enable_pin")
                    .and_then(|v| v.as_u64())
                    .ok_or("enable pin is not a valid number")? as u8,
                step_pin: stepper_config.get("step_pin")
                    .and_then(|v| v.as_u64())
                    .ok_or("step pin is not a valid number")? as u8,
                dir_pin: stepper_config.get("dir_pin")
                    .and_then(|v| v.as_u64())
                    .ok_or("dir pin is not a valid number")? as u8,
                hold_time_us,
            },
            step_length,
            directions_mapping,
        })
    }
}