#include <memory>
#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "linux_ipc_connection.hpp"

using namespace linux_mcu_ipc;
// using namespace mcu_client_utl;

TEST(ut_linux_ipc_connection, ctor_dtor) {
	// GIVEN
	const std::string uart_path("/dev/ttyACM0");
	const LinuxIpcConnection::Baud baud(LinuxIpcConnection::Baud::BAUD9600);
	const UartIpcData head("MSG_HEADER");
	const UartIpcData tail("MSG_TAIL");
	const std::size_t max_buff_size(1000UL);

	// WHEN
	std::unique_ptr<LinuxIpcConnection> instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		instance_ptr = std::unique_ptr<LinuxIpcConnection>(
			new LinuxIpcConnection(
				uart_path,
				baud,
				head,
				tail,
				max_buff_size
			)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(instance_ptr = nullptr);
}