use std::{collections::HashMap, time::Duration};

use movement_data::Vector;
use uart_port::UartPort;
use movement_service_client::{
    MovementApiRequest,
    MovementServiceClient,
    ProtoRequestSerializer,
    ProtoResponseParser,
    ServiceClient
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

    let speed = 50.0;
    let dx = 10.0;
    let dy = 10.0;
    let dz = 5.0;
    let step_length = 0.005;
    let hold_time_us = 100;
    let directions_mapping = HashMap::from(
        [
            ("NEGATIVE".to_string(), "CW".to_string()),
            ("POSITIVE".to_string(), "CCW".to_string()),
        ]
    );

    let x_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 17,
            step_pin: 16,
            dir_pin: 15,
            hold_time_us,
        },
        step_length,
        directions_mapping: directions_mapping.clone(),
    };
    let y_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 12,
            step_pin: 11,
            dir_pin: 10,
            hold_time_us,
        },
        step_length,
        directions_mapping: directions_mapping.clone(),
    };
    let z_config = movement_data::AxisConfig {
        stepper_config: movement_data::PicoStepperConfig {
            enable_pin: 8,
            step_pin: 7,
            dir_pin: 6,
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
            destination: Vector::new(dx, 0.0, 0.0),
            speed,
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(0.0, dy, 0.0),
            speed,
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(0.0, 0.0, dz),
            speed,
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(-dx, -dy, 0.0),
            speed,
        },
        MovementApiRequest::LinearMovement {
            destination: Vector::new(0.0, 0.0, -dz),
            speed,
        },
    ];
    for test_request in test_requests.iter() {
        println!("request: {:?}", test_request);
        let response = client.run_request(test_request);
        match response {
            Ok(_) => println!("request processed, response: {:?}", response),
            Err(e) => println!("error processing request: {e}"),
        }
    }
}