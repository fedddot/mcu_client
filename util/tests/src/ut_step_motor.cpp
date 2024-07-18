#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "gpo_proxy.hpp"
#include "json_data_parser.hpp"
#include "json_data_serializer.hpp"
#include "step_motor.hpp"
#include "uart_connection.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

enum: int {
	LH_GPIO_ID = 10,
	LL_GPIO_ID = 11,
	RH_GPIO_ID = 12,
	RL_GPIO_ID = 13,
};

TEST(ut_step_motor, ctor_dtor) {
	// GIVEN
	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
	GpoProxy::McuClient client(&connection);
	JsonDataParser parser;
	JsonDataSerializer serializer;
	GpoProxy lh(LH_GPIO_ID, &client, parser, serializer);
	GpoProxy ll(LL_GPIO_ID, &client, parser, serializer);
	GpoProxy rh(RH_GPIO_ID, &client, parser, serializer);
	GpoProxy rl(RL_GPIO_ID, &client, parser, serializer);


	// WHEN
	StepMotor *instance_ptr(nullptr);

	// THEN
	ASSERT_NO_THROW(
		(
			instance_ptr = new StepMotor(&lh, &ll, &rh, &rl)
		)
	);
	ASSERT_NE(nullptr, instance_ptr);
	ASSERT_NO_THROW(delete instance_ptr);

	instance_ptr = nullptr;
}

// TEST(ut_step_motor, state_sanity) {
// 	// GIVEN
// 	UartConnection connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD9600, 1000, "MSG_HEADER", "MSG_TAIL");
// 	GpiProxy::McuClient client(&connection);
// 	JsonDataParser parser;
// 	JsonDataSerializer serializer;


// 	// WHEN
// 	GpiProxy instance(s_testGpiId, &client, parser, serializer);
// 	GpiProxy::State result(GpiProxy::State::LOW);

// 	// THEN
// 	ASSERT_NO_THROW(result = instance.state());
// 	ASSERT_EQ(GpiProxy::State::HIGH, result);
// }