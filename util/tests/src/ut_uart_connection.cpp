#include <iostream>
#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "uart.hpp"
#include "uart_connection.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

TEST(ut_uart_connection, ctor_dtor) {
	// GIVEN
	Uart uart("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 100);

	// WHEN
	UartConnection *instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		(
			instance_ptr = new UartConnection(
				&uart,
				1000,
				"MSG_HEADER",
				"MSG_TAIL"
			)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

TEST(ut_uart_connection, run_sanity) {
	// GIVEN
	Uart uart("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 100);
	const mcu_client::ClientData test_data("{\"ctor_id\" : 1, \"gpio_id\" : 25}");
	
	// WHEN
	UartConnection instance(
		&uart,
		1000,
		"MSG_HEADER",
		"MSG_TAIL"
	);
	mcu_client::ClientData result("");

	// THEN
	ASSERT_NO_THROW(instance.send(test_data));
	ASSERT_NO_THROW(result = instance.read());
	std::cout << result << std::endl;
}