#include "step_motor.hpp"

using namespace mcu_client_utl;

const StepMotor::MotorStates mcu_client_utl::StepMotor::s_states {
	MotorState(
		{
			{Shoulder::LH, Gpo::State::HIGH},
			{Shoulder::LL, Gpo::State::LOW},
			{Shoulder::RH, Gpo::State::HIGH},
			{Shoulder::RL, Gpo::State::HIGH}
		}
	),
	MotorState(
		{
			{Shoulder::LH, Gpo::State::HIGH},
			{Shoulder::LL, Gpo::State::LOW},
			{Shoulder::RH, Gpo::State::HIGH},
			{Shoulder::RL, Gpo::State::HIGH}
		}
	),
	MotorState(
		{
			{Shoulder::LH, Gpo::State::HIGH},
			{Shoulder::LL, Gpo::State::LOW},
			{Shoulder::RH, Gpo::State::HIGH},
			{Shoulder::RL, Gpo::State::HIGH}
		}
	),
	MotorState(
		{
			{Shoulder::LH, Gpo::State::HIGH},
			{Shoulder::LL, Gpo::State::LOW},
			{Shoulder::RH, Gpo::State::HIGH},
			{Shoulder::RL, Gpo::State::HIGH}
		}
	)
};