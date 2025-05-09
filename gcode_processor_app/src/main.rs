use std::time::Duration;

use gcode_processor::GcodeProcessor;
use movement_service_client::{JsonRequestSerializer, JsonResponseParser, MovementServiceClient};
use uart_port::UartPort;
use uart_sized_package_reader_writer::{DefaultSizeDecoder, DefaultSizeEncoder, UartSizedPackageReader, UartSizedPackageWriter};

fn main() {
    let uart_port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let response_timeout = Duration::from_secs(10);
    let preamble = b"MSG_PREAMBLE";
    let encoded_length = 4;

    let uart_port = UartPort::new(uart_port_name, baud_rate, response_timeout).unwrap();
    let uart_reader = UartSizedPackageReader::new(
        &uart_port,
        preamble,
        Box::new(DefaultSizeDecoder::new(encoded_length)),
    );
    let uart_writer = UartSizedPackageWriter::new(
        &uart_port,
        preamble,
        Box::new(DefaultSizeEncoder::new(encoded_length)),
    );
    let movement_service_client = MovementServiceClient::new(
        Box::new(uart_reader),
        Box::new(uart_writer),
        Box::new(JsonRequestSerializer),
        Box::new(JsonResponseParser),
    );
    let mut processor = GcodeProcessor::new(
        60.0,
        30.0,
        Box::new(movement_service_client),
    );
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <gcode line>", args[0]);
        std::process::exit(1);
    }
    let gcode_line = &args[1];
    let result = processor.process(gcode_line);
    match result {
        Ok(_) => (),
        Err(msg) => {
            eprintln!("gcode processor failed to process the command: {gcode_line}, what: {msg}");
            std::process::exit(1);
        },
    }
}
