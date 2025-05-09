use super::*;
use mockall::mock;
use movement_data::Axis;

#[test]
fn sanity() {
    // THEN
    run_sanity_test_case(
        "G00 X1.2 Y3.4 Z5.6",
        7.8,
        9.10,
        move |request| {
            let target_vector = Vector::<f32>::new(1.2, 3.4, 5.6);
            let fast_speed = 7.8;
            let MovementType::Linear(movement_data) = &request.movement_type else {
                panic!("non-linear movement request");
            };
            assert_eq!(fast_speed, movement_data.speed);
            [Axis::X, Axis::Y, Axis::Z].iter().for_each(|axis| {
                assert_eq!(movement_data.destination.get(axis), target_vector.get(axis));
            });
            Ok(MovementManagerResponse { code: ResultCode::Ok, message: None })
        }
    );
    run_sanity_test_case(
        "G01 X1.2 Y3.4 F11.12",
        7.8,
        9.10,
        move |request| {
            let target_vector = Vector::<f32>::new(1.2, 3.4, 0.0);
            let expected_speed = 11.12;
            let MovementType::Linear(movement_data) = &request.movement_type else {
                panic!("non-linear movement request");
            };
            assert_eq!(expected_speed, movement_data.speed);
            [Axis::X, Axis::Y, Axis::Z].iter().for_each(|axis| {
                assert_eq!(movement_data.destination.get(axis), target_vector.get(axis));
            });
            Ok(MovementManagerResponse { code: ResultCode::Ok, message: None })
        }
    );
}

fn run_sanity_test_case<T>(gcode_line: &str, fast_speed: f32, default_speed: f32, client_callback: T)
where
    T: FnMut(&MovementManagerRequest) -> Result<MovementManagerResponse, String> + Send + 'static,
{
    // WHEN
    let mut mock_service_client = MockServiceClient::default();
    mock_service_client
        .expect_run_request()
        .returning(client_callback);
    let mut instance = GcodeProcessor::new(
        fast_speed,
        default_speed,
        Box::new(mock_service_client),
    );

    // THEN
    let result = instance.process(gcode_line);
    assert!(result.is_ok());
}

mock! {
    pub ServiceClient {}

    impl ServiceClient<MovementManagerRequest, MovementManagerResponse, String> for ServiceClient {
        fn run_request(&mut self, request: &MovementManagerRequest) -> Result<MovementManagerResponse, String>;
    }
}