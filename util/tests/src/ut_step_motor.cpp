#include <memory>
#include <string>
#include <unistd.h>

#include "gtest/gtest.h"

#include "gpo_proxy.hpp"
#include "decoding_data_parser.hpp"
#include "encoding_data_serializer.hpp"
#include "step_motor.hpp"
#include "uart_connection.hpp"

using namespace mcu_client_utl;
using namespace mcu_server_utl;

enum: int {
	LH_GPIO_ID = 6,
	LL_GPIO_ID = 7,
	RH_GPIO_ID = 8,
	RL_GPIO_ID = 9,
};

static UartConnection s_connection("/dev/ttyACM0", UartConnection::UartBaud::BAUD115200, 30000, "MSG_HEADER", "MSG_TAIL");

// TEST(ut_step_motor, ctor_dtor) {
// 	// GIVEN
// 	GpoProxy::McuClient client(&s_connection);
// 	const std::map<std::string, std::string> conversion_map {
//         {"ctor_id", "0"},
//         {"gpio_id", "1"},
//         {"gpio_dir", "2"},
//         {"gpio_state", "3"},
//         {"delay_ms", "4"},
//         {"tasks", "5"},
//         {"result", "6"},
//         {"reports", "7"},
//         {"what", "8"}
//     };
// 	DecodingDataParser parser(conversion_map);
// 	EncodingDataSerializer serializer(conversion_map);

// 	// WHEN
// 	std::unique_ptr<StepMotor> instance_ptr(nullptr);

// 	// THEN
// 	ASSERT_NO_THROW(
// 		(
// 			instance_ptr = std::make_unique<StepMotor>(LH_GPIO_ID, LL_GPIO_ID, RH_GPIO_ID, RL_GPIO_ID, 10, &client, parser, serializer)
// 		)
// 	);
// 	ASSERT_NE(nullptr, instance_ptr);
// 	ASSERT_NO_THROW(instance_ptr = nullptr);
// }

TEST(ut_step_motor, step_sanity) {
	// GIVEN
	GpoProxy::McuClient client(&s_connection);
	const std::map<std::string, std::string> conversion_map {
		{"ctor_id", "0"},
		{"gpio_id", "1"},
		{"gpio_dir", "2"},
		{"gpio_state", "3"},
		{"delay_ms", "4"},
		{"tasks", "5"},
		{"result", "6"},
		{"reports", "7"},
        {"what", "8"}
	};
	DecodingDataParser parser(conversion_map);
	EncodingDataSerializer serializer(conversion_map);

	// WHEN
	StepMotor instance(LH_GPIO_ID, LL_GPIO_ID, RH_GPIO_ID, RL_GPIO_ID, 20, &client, parser, serializer);
	int steps_num(30);

	// THEN
	std::cout << std::endl << "Running CW steps: " << std::to_string(steps_num) << std::endl;
	ASSERT_NO_THROW(instance.steps(steps_num, StepMotor::Direction::CW));
	std::cout << std::endl << "Running CCW steps: " << std::to_string(steps_num) << std::endl;
	ASSERT_NO_THROW(instance.steps(steps_num, StepMotor::Direction::CCW));
}