use std::fs::File;
use std::{collections::HashMap};
use std::io::{BufRead, BufReader, Read};
use std::time::Duration;

use gcode_processor::{CoordinatesType, GcodeProcessor, GcodeProcessorState, StateStorage};
use movement_data::{Axis, AxisConfig, PicoStepperConfig, Vector};
use movement_service_client::{MovementServiceClient, ProtoRequestSerializer, ProtoResponseParser};
use serde_json::{json, Value};
use uart_port::UartPort;
use uart_sized_package_reader_writer::{DefaultSizeDecoder, DefaultSizeEncoder, UartSizedPackageReader, UartSizedPackageWriter};

use crate::configurer::JsonFileConfigurer;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("gcode_processor_app")
        .about("Processes G-code lines")
        .arg(Arg::new("config_path")
            .short('c')
            .long("config")
            .help("Path to the config JSON file")
            .required(true)
        )
        .arg(Arg::new("gcode_file")
            .help("Path to the G-code file to process")
            .required(true)
            .index(1),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config_path").expect("required argument");
    let gcode_file = matches.get_one::<String>("gcode_file").expect("required argument");
    let configurer = JsonFileConfigurer::new(config_path);
    let config = configurer
        .config()
        .unwrap_or_else(|err| {
            eprintln!("an error occured on reading config at {config_path}: {err}");
            std::process::exit(-1);
        });

    let uart_port = UartPort::new(
        &config.uart_port.port_name,
        config.uart_port.baud,
        Duration::from_secs(config.uart_port.response_timeout_s as u64)
    ).unwrap_or_else(|err| {
        eprintln!("an error occured on creating UART port: {err}");
        std::process::exit(-1);
    });
    let uart_reader = UartSizedPackageReader::new(
        &uart_port,
        config.uart_package.preamble.as_bytes(),
        Box::new(DefaultSizeDecoder::new(config.uart_package.size_field_length as usize)),
    );
    let uart_writer = UartSizedPackageWriter::new(
        &uart_port,
        config.uart_package.preamble.as_bytes(),
        Box::new(DefaultSizeEncoder::new(config.uart_package.size_field_length as usize)),
    );
    let movement_service_client = MovementServiceClient::new(
        Box::new(uart_reader),
        Box::new(uart_writer),
        Box::new(ProtoRequestSerializer),
        Box::new(ProtoResponseParser),
    );
    let state_storage = JsonStateStorage::new(&config.state_storage.file_path);
    let mut processor = GcodeProcessor::new(
        6.0,
        3.0,
        Box::new(movement_service_client),
        &generate_axes_cfg(),
        Box::new(state_storage),
    );

    let gcode_lines = read_gcode_lines(gcode_file).unwrap();
    for gcode_line in &gcode_lines {
        let result = processor.process(gcode_line);
        match result {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("gcode processor failed to process the command: {gcode_line}, what: {msg}");
                std::process::exit(-1);
            },
        }
    }
    std::process::exit(0);
}

fn generate_axes_cfg() -> HashMap<Axis, AxisConfig> {
    let step_length = 0.005;
    let hold_time_us = 10;
    let directions_mapping = HashMap::from([
        ("POSITIVE".to_string(), "CCW".to_string()),
        ("NEGATIVE".to_string(), "CW".to_string()),
    ]);
    HashMap::from([
        (
            Axis::X,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 17,
                    step_pin: 16,
                    dir_pin: 15,
                    hold_time_us,
                },
                step_length,
                directions_mapping: directions_mapping.clone(),
            }
        ),
        (
            Axis::Y,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 12,
                    step_pin: 11,
                    dir_pin: 10,
                    hold_time_us,
                },
                step_length,
                directions_mapping: directions_mapping.clone(),
            }
        ),
        (
            Axis::Z,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 8,
                    step_pin: 7,
                    dir_pin: 6,
                    hold_time_us,
                },
                step_length,
                directions_mapping: HashMap::from([
                    ("POSITIVE".to_string(), "CW".to_string()),
                    ("NEGATIVE".to_string(), "CCW".to_string()),
                ]),
            }
        ),
    ])
}

fn read_gcode_lines(gcode_file_path: &str) -> Result<Vec<String>, String> {
    
    let file = File::open(gcode_file_path)
        .map_err(|e| format!("failed to open G-code file: {}", e))?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(|line_result| {
            let Ok(line) = line_result else {
                return None;
            };
            let re = regex::Regex::new(r"(?i)\bg\d{1,3}\b").unwrap();
            match re.is_match(&line) {
                true => Some(line),
                false => None,
            }
        })
        .collect();
    Ok(lines)
}

struct JsonStateStorage {
    file_path: String,
}

impl JsonStateStorage {
    fn new(file_path: &str) -> Self {
        if !std::path::Path::new(file_path).exists() {
            if let Some(parent) = std::path::Path::new(file_path).parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(file_path).unwrap();
            Self::write_state(file_path, &GcodeProcessorState::default())
                .expect("failed to initialize state file");
        }
        Self { file_path: file_path.to_string() }
    }

    fn write_state(file_path: &str, state: &GcodeProcessorState) -> Result<(), String> {
        let coordinates_type_json = match state.coordinates_type {
            CoordinatesType::Absolute => json!("absolute"),
            CoordinatesType::Relative => json!("relative"),
        };
        let json_data = json!({
            "coordinates_type": coordinates_type_json,
            "current_position": Self::vector_to_json_object(&state.current_position),
        });
        let json_string = serde_json::to_string(&json_data)
            .map_err(|e| format!("failed to serialize state to JSON: {}", e))?;
        std::fs::write(file_path, json_string)
            .map_err(|e| format!("failed to write state to file: {}", e))
    }

    fn read_state(file_path: &str) -> Result<GcodeProcessorState, String> {
        let mut file = std::fs::File::open(file_path)
            .map_err(|e| format!("failed to open state file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("failed to read state file: {}", e))?;
        let json_data: Value = serde_json::from_str(&contents)
            .map_err(|e| format!("failed to parse state file: {}", e))?;
        let current_position_json = Self::read_json_value_from_object(&json_data, "current_position")?;
        let current_position = Self::json_object_to_vector(&current_position_json)?;
        let coordinates_type_str = Self::read_json_value_from_object(&json_data, "coordinates_type")?;
        let coordinates_type = match coordinates_type_str.as_str() {
            Some("absolute") => CoordinatesType::Absolute,
            Some("relative") => CoordinatesType::Relative,
            _ => return Err("invalid coordinates type in state file".to_string()),
        };
        Ok(GcodeProcessorState {
            current_position,
            coordinates_type,
        })
    }

    fn read_json_value_from_object(json_data: &Value, key: &str) -> Result<Value, String> {
        let Some(result) = json_data.get(key) else {
            return Err(format!("missing '{key}' in JSON data"));
        };
        Ok(result.clone())
    }

    fn json_object_to_vector(json_object: &Value) -> Result<Vector<f32>, String> {
        let mut vector = Vector::default();
        for (axis_str, axis) in [("x", Axis::X), ("y", Axis::Y), ("z", Axis::Z)] {
            let value = Self::read_json_value_from_object(json_object, axis_str)?
                .as_f64()
                .ok_or_else(|| format!("invalid vector value for axis '{axis_str}'"))?;
            vector.set(&axis, value as f32);
        }
        Ok(vector)
    }

    fn vector_to_json_object(vector: &Vector<f32>) -> Value {
        let mut json_value = json!({});
        for (axis_str, axis) in [("x", Axis::X), ("y", Axis::Y), ("z", Axis::Z)] {
            json_value[axis_str] = json!(vector.get(&axis));
        }
        json_value
    }
}

impl StateStorage for JsonStateStorage {
    fn read_state(&self) -> Result<GcodeProcessorState, String> {
        Self::read_state(&self.file_path)
    }

    fn write_state(&mut self, state: &GcodeProcessorState) -> Result<(), String> {
        Self::write_state(&self.file_path, state)
    }
}

mod config;
mod configurer;