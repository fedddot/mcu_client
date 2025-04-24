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
    let mut client = StepperServiceClient::new(
        Box::new(uart_reader),
            Box::new(uart_writer),
        Box::new(JsonRequestSerializer),
        Box::new(JsonResponseParser),
    );

    let dur = Duration::from_millis(1);
    let motor1_id = "stepper_1".to_string();
    let motor2_id = "stepper_2".to_string();
    let motor3_id = "stepper_3".to_string();

    let test_requests = [
        StepperMotorRequest {
            motor_id: motor1_id.clone(),
            direction: StepperMotorDirection::CCW,
            steps_number: 500,
            step_duration: dur,
        },
        StepperMotorRequest {
            motor_id: motor1_id.clone(),
            direction: StepperMotorDirection::CW,
            steps_number: 500,
            step_duration: dur,
        },
        StepperMotorRequest {
            motor_id: motor2_id.clone(),
            direction: StepperMotorDirection::CCW,
            steps_number: 500,
            step_duration: dur,
        },
        StepperMotorRequest {
            motor_id: motor2_id.clone(),
            direction: StepperMotorDirection::CW,
            steps_number: 500,
            step_duration: dur,
        },
        StepperMotorRequest {
            motor_id: motor3_id.clone(),
            direction: StepperMotorDirection::CCW,
            steps_number: 500,
            step_duration: dur,
        },
        StepperMotorRequest {
            motor_id: motor3_id.clone(),
            direction: StepperMotorDirection::CW,
            steps_number: 500,
            step_duration: dur,
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
