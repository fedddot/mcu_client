use super::*;
use mockall::mock;
use movement_data::Axis;

#[test]
fn sanity_linear_movements() {
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

#[test]
fn sanity_control() {
    // GIVEN
    let mock_service_client = MockServiceClient::default();
    let g90_line = "G90";
    let g91_line = "G91";
    
    // WHEN
    let mut instance = GcodeProcessor::new(
        60.0,
        30.0,
        Box::new(mock_service_client),
    );

    // THEN
    assert_eq!(instance.state().coordinates_type, CoordinatesType::Absolute);
    
    let result = instance.process(g91_line);
    assert!(result.is_ok());
    assert_eq!(instance.state().coordinates_type, CoordinatesType::Relative);
    
    let result = instance.process(g90_line);
    assert!(result.is_ok());
    assert_eq!(instance.state().coordinates_type, CoordinatesType::Absolute);
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