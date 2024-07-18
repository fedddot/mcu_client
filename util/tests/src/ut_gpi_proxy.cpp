#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "gpi_proxy.hpp"
#include "json_data_parser.hpp"
#include "json_data_serializer.hpp"
#include "uart.hpp"
#include "uart_connection.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

TEST(ut_gpi_proxy, ctor_dtor) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpiProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;


	// WHEN
	GpiProxy *instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		(
			instance_ptr = new GpiProxy(25, &client, parser, serializer)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

TEST(ut_gpi_proxy, run_sanity) {
	// // GIVEN
	// const mcu_client::ClientData test_data("MSG_HEADER{\"ctor_id\" : 0, \"gpio_dir\" : 1, \"gpio_id\" : 25}MSG_TAIL");
	// bool data_received(false);
	// std::mutex mux;
	// std::condition_variable cond;

	// CustomListener<mcu_client::ClientData> test_listener(
	// 	[&data_received, &mux, &cond](const mcu_client::ClientData& data) {
	// 		std::unique_lock lock(mux);
	// 		std::cout << "data received: " << data << std::endl;
	// 		data_received = true;
	// 		cond.notify_one();
	// 		std::cout << "notified" << std::endl;
	// 	}
	// );
	
	// // WHEN
	// Uart instance("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 100);
	// mcu_client::ClientData result("");

	// // THEN
	// ASSERT_NO_THROW(instance.start_listening(test_listener));
	// ASSERT_TRUE(instance.is_listening());
	
	// std::unique_lock lock(mux);
	// ASSERT_NO_THROW(instance.send(test_data));
	
	// cond.wait(lock);
	// ASSERT_NO_THROW(instance.stop_listening());
	// ASSERT_FALSE(instance.is_listening());
}