use super::*;

#[test]
fn sanity() {
    let mut builder = IpcReaderBuilder::<MovementApiRequest, MovementApiResponse, String>::default();
    builder.set_raw_data_reader(Box::new(MockIpcReader::new()));
    builder.set_raw_data_parser(Box::new(|data| {
        Ok(MovementApiResponse {
            status: StatusCode::Success,
            message: Some(format!("Parsed data: {:?}", data)),
        })
    }));
    let reader = builder.build().unwrap();
    assert!(reader.is_ok());
}