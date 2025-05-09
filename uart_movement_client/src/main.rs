use std::time::Duration;

use movement_data::{LinearMovementData, MovementType, Vector};
use uart_port::UartPort;
use movement_service_client::{
    JsonRequestSerializer, JsonResponseParser, MovementManagerRequest, MovementServiceClient, ServiceClient
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
        Box::new(JsonRequestSerializer),
        Box::new(JsonResponseParser),
    );

    let speed = 130.0;
    let dx = 20.0;
    let dy = 30.0;
    let dz = 40.0;
    let test_requests = [
        MovementManagerRequest {
            movement_type: MovementType::Linear(
                LinearMovementData {
                    destination: Vector::new(dx, dy, dz),
                    speed,
                }
            ),
        },
        MovementManagerRequest {
            movement_type: MovementType::Linear(
                LinearMovementData {
                    destination: Vector::new(-dx, -dy, dz),
                    speed,
                }
            ),
        },
    ];
    for test_request in test_requests.iter() {
        println!("request: {:?}", test_request);
        let response = client.run_request(test_request);
        let response = response.unwrap();
        println!("response: {:?}", response);
        println!(" ");
    }
}
