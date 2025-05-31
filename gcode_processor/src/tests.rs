use super::*;
use mockall::mock;
use movement_data::{Axis, PicoStepperConfig};

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
            let MovementApiRequest::LinearMovement { destination, speed } = request else {
                return Ok(MovementApiResponse { status: StatusCode::Success, message: None });
            };
            assert_eq!(fast_speed, *speed);
            [Axis::X, Axis::Y, Axis::Z].iter().for_each(|axis| {
                assert_eq!(destination.get(axis), target_vector.get(axis));
            });
            Ok(MovementApiResponse { status: StatusCode::Success, message: None })
        }
    );
    run_sanity_test_case(
        "G01 X1.2 Y3.4 F11.12",
        7.8,
        9.10,
        move |request| {
            let target_vector = Vector::<f32>::new(1.2, 3.4, 0.0);
            let expected_speed = 11.12;
            let MovementApiRequest::LinearMovement { destination, speed } = request else {
                return Ok(MovementApiResponse { status: StatusCode::Success, message: None });
            };
            assert_eq!(expected_speed, *speed);
            [Axis::X, Axis::Y, Axis::Z].iter().for_each(|axis| {
                assert_eq!(destination.get(axis), target_vector.get(axis));
            });
            Ok(MovementApiResponse { status: StatusCode::Success, message: None })
        }
    );
}

#[test]
fn sanity_control() {
    // GIVEN
    let g90_line = "G90";
    let g91_line = "G91";
    let state = std::sync::Arc::new(std::sync::Mutex::new(GcodeProcessorState::default()));
    let mut state_storage = MockStateStorage::default();
    let mut mock_service_client = MockServiceClient::default();

    // WHEN
    let state_for_reading = state.clone();
    state_storage
        .expect_read_state()
        .returning(move || Ok(state_for_reading.lock().unwrap().clone()));
    let state_for_writing = state.clone();
    state_storage
        .expect_write_state()
        .returning(move |new_state| {
            let mut state = state_for_writing.lock().unwrap();
            *state = new_state.clone();
            Ok(())
        });
    mock_service_client
        .expect_run_request()
        .returning(|_| Ok(MovementApiResponse { status: StatusCode::Success, message: None }));
    
    // WHEN
    let mut instance = GcodeProcessor::new(
        60.0,
        30.0,
        Box::new(mock_service_client),
        &generate_axes_cfg(),
        Box::new(state_storage),
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
    T: FnMut(&MovementApiRequest) -> Result<MovementApiResponse, String> + Send + 'static,
{
    // GIVEN
    let state = std::sync::Arc::new(std::sync::Mutex::new(GcodeProcessorState::default()));
    let mut state_storage = MockStateStorage::default();

    // WHEN
    let state_for_reading = state.clone();
    state_storage
        .expect_read_state()
        .returning(move || Ok(state_for_reading.lock().unwrap().clone()));
    let state_for_writing = state.clone();
    state_storage
        .expect_write_state()
        .returning(move |new_state| {
            let mut state = state_for_writing.lock().unwrap();
            *state = new_state.clone();
            Ok(())
        });
    let mut mock_service_client = MockServiceClient::default();
    mock_service_client
        .expect_run_request()
        .returning(client_callback);
    let mut instance = GcodeProcessor::new(
        fast_speed,
        default_speed,
        Box::new(mock_service_client),
        &generate_axes_cfg(),
        Box::new(state_storage),
    );

    // THEN
    let result = instance.process(gcode_line);
    assert!(result.is_ok());
}

fn generate_axes_cfg() -> HashMap<Axis, AxisConfig> {
    let step_length = 0.01;
    let hold_time_us = 1000;
    let directions_mapping = HashMap::from([
        ("POSITIVE".to_string(), "CCW".to_string()),
        ("NEGATIVE".to_string(), "CW".to_string()),
    ]);
    HashMap::from([
        (
            Axis::X,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 3,
                    step_pin: 4,
                    dir_pin: 5,
                    hold_time_us,
                },
                step_length,
                directions_mapping: directions_mapping.clone(),
            }
        ),
        (
            Axis::Y,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 6,
                    step_pin: 7,
                    dir_pin: 8,
                    hold_time_us,
                },
                step_length,
                directions_mapping: directions_mapping.clone(),
            }
        ),
        (
            Axis::Z,
            AxisConfig {
                stepper_config: PicoStepperConfig {
                    enable_pin: 9,
                    step_pin: 10,
                    dir_pin: 11,
                    hold_time_us,
                },
                step_length,
                directions_mapping: directions_mapping.clone(),
            }
        ),
    ])
}

mock! {
    pub ServiceClient {}

    impl ServiceClient<MovementApiRequest, MovementApiResponse, String> for ServiceClient {
        fn run_request(&mut self, request: &MovementApiRequest) -> Result<MovementApiResponse, String>;
    }
}

mock! {
    pub StateStorage {}

    impl StateStorage for StateStorage {
        fn read_state(&self) -> Result<GcodeProcessorState, String>;
        fn write_state(&mut self, state: &GcodeProcessorState) -> Result<(), String>;
    }
}