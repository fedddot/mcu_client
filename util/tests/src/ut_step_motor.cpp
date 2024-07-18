#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "gpi.hpp"
#include "step_motor.hpp"

using namespace mcu_client_utl;

TEST(ut_step_motor, ctor_dtor) {
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
			instance_ptr = new GpiProxy(s_testGpiId, &client, parser, serializer)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

TEST(ut_step_motor, state_sanity) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpiProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;


	// WHEN
	GpiProxy instance(s_testGpiId, &client, parser, serializer);
	GpiProxy::State result(GpiProxy::State::LOW);

	// THEN
	ASSERT_NO_THROW(result = instance.state());
	ASSERT_EQ(GpiProxy::State::HIGH, result);
}