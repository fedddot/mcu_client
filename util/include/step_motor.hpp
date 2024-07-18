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
		using StateIter = typename MotorStates::const_iterator;
		using MotorGpos = std::map<Shoulder, Gpo *>;

		MotorGpos m_gpos;
		StateIter m_curr_state;
		void apply_state(const MotorState& state);
		static const MotorStates s_states;
		static MotorGpos init_gpos(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo);
		static StateIter next_state_cw(const StateIter& curr_state);
		static StateIter next_state_ccw(const StateIter& curr_state);
	};

	inline StepMotor::StepMotor(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo): m_gpos(init_gpos(lh_gpo, ll_gpo, rh_gpo, rl_gpo)), m_curr_state(s_states.begin()) {
		apply_state(*m_curr_state);
	}

	inline void StepMotor::step(const Direction& dir) {
		StateIter next_state_iter(m_curr_state);
		switch (dir) {
		case Direction::CW:
			next_state_iter = next_state_cw(m_curr_state);
			break;
		case Direction::CCW:
			next_state_iter = next_state_ccw(m_curr_state);
			break;
		default:
			throw std::invalid_argument("unsupported direction received");
		}
		apply_state(*next_state_iter);
		m_curr_state = next_state_iter;
	}

	inline StepMotor::MotorGpos StepMotor::init_gpos(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo) {
		MotorGpos gpos {
			{Shoulder::LH, lh_gpo},
			{Shoulder::LL, ll_gpo},
			{Shoulder::RH, rh_gpo},
			{Shoulder::RL, rl_gpo}
		};
		for (auto item: gpos) {
			if (!(item.second)) {
				throw std::invalid_argument("invalid Gpo ptr received for shoulder " + std::to_string(static_cast<int>(item.first)));
			}
		}
		return gpos;
	}

	inline StepMotor::StateIter StepMotor::next_state_cw(const StateIter& curr_state) {
		const StateIter next_state_iter(curr_state + 1);
		if (s_states.end() == next_state_iter) {
			return s_states.begin();
		}
		return next_state_iter;
	}

	inline StepMotor::StateIter StepMotor::next_state_ccw(const StateIter& curr_state) {
		if (s_states.begin() == curr_state) {
			return s_states.begin() + MOTOR_STATES_NUMBER - 1;
		}
		return curr_state - 1;
	}
}
#endif // STEP_MOTOR_HPP