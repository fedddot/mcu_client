#include <condition_variable>
#include <iostream>
#include <mutex>
#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "custom_listener.hpp"
#include "uart.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

TEST(ut_uart, ctor_dtor) {
	// WHEN
	Uart *instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		(
			instance_ptr = new Uart("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 10000)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

TEST(ut_uart, run_sanity) {
	// GIVEN
	const mcu_client::ClientData test_data("MSG_HEADER{\"ctor_id\" : 0, \"gpio_dir\" : 1, \"gpio_id\" : 25}MSG_TAIL");
	bool data_received(false);
	std::mutex mux;
	std::condition_variable cond;

	CustomListener<mcu_client::ClientData> test_listener(
		[&data_received, &mux, &cond](const mcu_client::ClientData& data) {
			std::unique_lock lock(mux);
			std::cout << "data received: " << data << std::endl;
			data_received = true;
			cond.notify_one();
			std::cout << "notified" << std::endl;
		}
	);
	
	// WHEN
	Uart instance("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 100);
	mcu_client::ClientData result("");

	// THEN
	ASSERT_NO_THROW(instance.start_listening(test_listener));
	ASSERT_TRUE(instance.is_listening());
	
	std::unique_lock lock(mux);
	ASSERT_NO_THROW(instance.send(test_data));
	
	cond.wait(lock);
	ASSERT_NO_THROW(instance.stop_listening());
	ASSERT_FALSE(instance.is_listening());
}