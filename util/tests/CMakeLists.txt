add_executable(
	mcu_client_util_tests
	${CMAKE_CURRENT_LIST_DIR}/src/main.cpp
	${CMAKE_CURRENT_LIST_DIR}/src/ut_uart.cpp
	${CMAKE_CURRENT_LIST_DIR}/src/ut_uart_connection.cpp
	${CMAKE_CURRENT_LIST_DIR}/src/ut_gpi_proxy.cpp
)

target_link_libraries(
	mcu_client_util_tests
	PRIVATE
	uart
	custom_listener
	uart_connection

    gpi_proxy
    json_data_parser
    json_data_serializer

	gtest
)
