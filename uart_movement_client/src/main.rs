use std::{collections::HashMap, time::Duration};

use movement_data::{MovementApiResponse, Vector};
use uart_port::UartPort;
use movement_service_client::{
    DataTransformer, MovementApiRequest, MovementServiceClient, ServiceClient
};
use uart_sized_package_reader_writer::{
    DefaultSizeDecoder,
    DefaultSizeEncoder,
    UartSizedPackageReader,
    UartSizedPackageWriter
};

fn main() {
    let uart_port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let response_timeout = Duration::from_secs(10);
    let preamble = b"MSG_PREAMBLE";
    let encoded_size_len = 4;

    let uart_port = UartPort::new(uart_port_name, baud_rate, response_timeout).unwrap();
    let uart_reader = UartSizedPackageReader::new(
        &uart_port,
        preamble,
        Box::new(DefaultSizeDecoder::new(encoded_size_len)),
    );
    let uart_writer = UartSizedPackageWriter::new(
        &uart_port,
        preamble,
        Box::new(DefaultSizeEncoder::new(encoded_size_len)),
    );
    let mut client = MovementServiceClient::new(
        Box::new(uart_reader),
            Box::new(uart_writer),
        Box::new(ProtoRequestSerializer),
        Box::new(ProtoResponseParser),
    );

    let speed = 130.0;
    let dx = 20.0;
    let dy = 30.0;
    let dz = 40.0;
    let step_length = 0.01;
    let hold_time_us = 1;
    let directions_mapping = HashMap::from(
        [
            ("NEGATIVE".to_string(), "CW".to_string()),
            ("POSITIVE".to_string(), "CCW".to_string()),
        ]
    );

    let x_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 3,
            step_pin: 4,
            dir_pin: 5,
            hold_time_us,
        },
        step_length,
        directions_mapping: directions_mapping.clone(),
    };
    let y_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 6,
            step_pin: 7,
            dir_pin: 8,
            hold_time_us,
        },
        step_length,
        directions_mapping: directions_mapping.clone(),
    };
    let z_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 9,
            step_pin: 10,
            dir_pin: 11,
            hold_time_us,
        },
        step_length,
        directions_mapping: directions_mapping.clone(),
    };


    let test_requests = [
        MovementApiRequest::Config {
            axes_configs: HashMap::from([
                (movement_data::Axis::X, x_config),
                (movement_data::Axis::Y, y_config),
                (movement_data::Axis::Z, z_config),
            ]),
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(dx, dy, dz),
            speed,
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(-dx, -dy, dz),
            speed,
        },
    ];
    for test_request in test_requests.iter() {
        println!("request: {:?}", test_request);
        let response = client.run_request(test_request);
        println!("response: {:?}", response);
        let response = response.unwrap();
        println!(" ");
    }
}

struct ProtoRequestSerializer;

impl DataTransformer<MovementApiRequest, Vec<u8>, String> for ProtoRequestSerializer {
    fn transform(&self, input: &MovementApiRequest) -> Result<Vec<u8>, String> {
        Ok(proto_transformers::serialize_movement_request(input))
    }
}

struct ProtoResponseParser;

impl DataTransformer<Vec<u8>, MovementApiResponse, String> for ProtoResponseParser {
    fn transform(&self, input: &Vec<u8>) -> Result<MovementApiResponse, String> {
        proto_transformers::parse_movement_response(input)
    }
}