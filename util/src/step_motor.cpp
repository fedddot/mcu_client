#include "step_motor.hpp"

using namespace mcu_client_utl;

const StepMotor::MotorStates mcu_client_utl::StepMotor::s_states {
	MotorState(
		{
			{Shoulder::LH, GpioState::HIGH},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::LOW}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::HIGH},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::HIGH},
			{Shoulder::RL, GpioState::LOW}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::HIGH},
			{Shoulder::RL, GpioState::LOW}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::HIGH},
			{Shoulder::RH, GpioState::HIGH},
			{Shoulder::RL, GpioState::LOW}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::HIGH},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::LOW}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::HIGH},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::HIGH}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::HIGH}
		}
	),
	MotorState(
		{
			{Shoulder::LH, GpioState::HIGH},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::HIGH}
		}
	)
};