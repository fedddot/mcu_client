add_executable(
	mcu_client_util_tests
	${CMAKE_CURRENT_LIST_DIR}/src/main.cpp
	# ${CMAKE_CURRENT_LIST_DIR}/src/ut_uart.cpp
	${CMAKE_CURRENT_LIST_DIR}/src/ut_uart_connection.cpp
	# ${CMAKE_CURRENT_LIST_DIR}/src/ut_gpi_proxy.cpp
	# ${CMAKE_CURRENT_LIST_DIR}/src/ut_gpo_proxy.cpp
	# ${CMAKE_CURRENT_LIST_DIR}/src/ut_step_motor.cpp
)

target_link_libraries(
	mcu_client_util_tests
	PRIVATE
	uart
	custom_listener
	uart_connection

    gpi_proxy
	gpo_proxy
    decoding_data_parser
	encoding_data_serializer

	step_motor

	gtest
)
