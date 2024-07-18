#ifndef	STEP_MOTOR_HPP
#define	STEP_MOTOR_HPP

#include "gpo.hpp"
#include <array>
#include <map>
#include <memory>
#include <stdexcept>
#include <string>

namespace mcu_client_utl {
	class StepMotor {
	public:
		using Gpo = mcu_task_engine::Gpo;
		enum class Direction: int {
			CW,
			CCW
		};

		StepMotor(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo);
		StepMotor(const StepMotor& other) = default;
		StepMotor& operator=(const StepMotor& other) = default;
		~StepMotor() noexcept = default;

		void step(const Direction& dir);
	private:
		enum class Shoulder: int {
			LH,
			LL,
			RH,
			RL
		};
		enum: int { MOTOR_STATES_NUMBER = 4 };
		
		using MotorState = std::map<Shoulder, Gpo::State>;
		using MotorStates = std::array<MotorState, MOTOR_STATES_NUMBER>;
		using CurrentStateIter = typename MotorStates::const_iterator;
		using MotorGpos = std::map<Shoulder, Gpo *>;

		MotorGpos m_gpos;
		CurrentStateIter m_curr_state;
		static const MotorStates s_states;
		static MotorGpos init_gpos(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo);
	};



	inline StepMotor::StepMotor(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo): m_gpos(init_gpos(lh_gpo, ll_gpo, rh_gpo, rl_gpo)), m_curr_state(s_states.begin()) {

	}

	inline StepMotor::MotorGpos StepMotor::init_gpos(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo) {

	}

	
}
#endif // STEP_MOTOR_HPP