#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "gpo_proxy.hpp"
#include "json_data_parser.hpp"
#include "json_data_serializer.hpp"
#include "uart.hpp"
#include "uart_connection.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

static const int s_testGpoId(25);

TEST(ut_gpo_proxy, ctor_dtor) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpoProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;


	// WHEN
	GpoProxy *instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		(
			instance_ptr = new GpoProxy(s_testGpoId, &client, parser, serializer)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

TEST(ut_gpo_proxy, state_sanity) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpoProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;


	// WHEN
	GpoProxy instance(s_testGpoId, &client, parser, serializer);
	GpoProxy::State result(GpoProxy::State::LOW);

	// THEN
	ASSERT_NO_THROW(result = instance.state());
	ASSERT_EQ(GpoProxy::State::LOW, result);
}

TEST(ut_gpo_proxy, set_state_sanity) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpoProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;


	// WHEN
	GpoProxy instance(s_testGpoId, &client, parser, serializer);
	GpoProxy::State result(GpoProxy::State::LOW);

	// THEN
	ASSERT_NO_THROW(instance.set_state(GpoProxy::State::HIGH));
	ASSERT_NO_THROW(result = instance.state());
	ASSERT_EQ(GpoProxy::State::HIGH, result);
	sleep(1);
	ASSERT_NO_THROW(instance.set_state(GpoProxy::State::LOW));
	ASSERT_NO_THROW(result = instance.state());
	ASSERT_EQ(GpoProxy::State::LOW, result);
}