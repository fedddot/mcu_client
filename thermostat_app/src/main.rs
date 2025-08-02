use std::time::Duration;

use thermo_data::ThermostatApiRequest;
use thermostat_service_client::{ProtoRequestSerializer, ProtoResponseParser, ServiceClient, ThermostatServiceClient};
use uart_port::UartPort;
use uart_sized_package_reader_writer::{DefaultSizeDecoder, DefaultSizeEncoder, UartSizedPackageReader, UartSizedPackageWriter};

use crate::configurer::JsonFileConfigurer;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("thermostat_app")
        .about("Processes G-code lines")
        .arg(Arg::new("config_path")
            .short('c')
            .long("config")
            .help("Path to the config JSON file")
            .required(true)
        )
        .arg(Arg::new("request_file_path")
            .short('r')
            .long("request")
            .help("Path to the JSON request file to process")
            .required(true)
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config_path").expect("required argument");
    let configurer = JsonFileConfigurer::new(config_path);
    let config = configurer
        .config()
        .unwrap_or_else(|err| {
            eprintln!("an error occured on reading config at {config_path}: {err}");
            std::process::exit(-1);
        });
    
    let request_file_path = matches.get_one::<String>("request_file_path").expect("required argument");
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
    let mut thermostat_service_client = ThermostatServiceClient::new(
        Box::new(uart_reader),
        Box::new(uart_writer),
        Box::new(ProtoRequestSerializer),
        Box::new(ProtoResponseParser),
    );
    let request_json = std::fs::read_to_string(request_file_path)
        .unwrap_or_else(|err| {
            eprintln!("an error occured on reading request file at {request_file_path}: {err}");
            std::process::exit(-1);
        });
    let request: ThermostatApiRequest = serde_json::from_str(&request_json)
        .unwrap_or_else(|err| {
            eprintln!("an error occured on deserializing request: {err}");
            std::process::exit(-1);
        });
    println!("Request: {request:?}");
    let response = thermostat_service_client.run_request(&request)
        .unwrap_or_else(|err| {
            eprintln!("an error occured on running request: {err}");
            std::process::exit(-1);
        });
    println!("Response: {response:?}");
    std::process::exit(0);
}

mod config;
mod configurer;