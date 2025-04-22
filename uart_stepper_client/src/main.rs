use std::time::Duration;

use uart_port::UartPort;
use stepper_service_client::{
    JsonRequestSerializer, JsonResponseParser, StepperMotorDirection, StepperMotorRequest, StepperServiceClient, ServiceClient
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
    let response_timeout = Duration::from_secs(3);
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
    let mut client = StepperServiceClient::new(
        Box::new(uart_reader),
            Box::new(uart_writer),
        Box::new(JsonRequestSerializer),
        Box::new(JsonResponseParser),
    );

    let test_requests = [
        StepperMotorRequest {
            direction: StepperMotorDirection::CCW,
            steps_number: 10,
            step_duration: Duration::from_millis(100),
        },
        StepperMotorRequest {
            direction: StepperMotorDirection::CW,
            steps_number: 20,
            step_duration: Duration::from_millis(50),
        },
        StepperMotorRequest {
            direction: StepperMotorDirection::CCW,
            steps_number: 100,
            step_duration: Duration::from_millis(10),
        },
        StepperMotorRequest {
            direction: StepperMotorDirection::CW,
            steps_number: 1000,
            step_duration: Duration::from_millis(1),
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
