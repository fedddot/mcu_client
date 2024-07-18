#ifndef	STEP_MOTOR_HPP
#define	STEP_MOTOR_HPP

#include <array>
#include <map>
#include <stdexcept>
#include <string>

#include "gpo.hpp"

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
		~StepMotor() noexcept;

		void step(const Direction& dir);
	private:
		enum class Shoulder: int {
			LH,
			LL,
			RH,
			RL
		};
		enum: int { MOTOR_STATES_NUMBER = 8 };
		
		using MotorState = std::map<Shoulder, Gpo::State>;
		using MotorStates = std::array<MotorState, MOTOR_STATES_NUMBER>;
		using MotorGpos = std::map<Shoulder, Gpo *>;

		MotorGpos m_gpos;
		int m_curr_state;
		void apply_state(const MotorState& state);
		static const MotorStates s_states;
		static MotorGpos init_gpos(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo);
		static int next_state_cw(int curr_state);
		static int next_state_ccw(int curr_state);
	};

	inline StepMotor::StepMotor(Gpo *lh_gpo, Gpo *ll_gpo, Gpo *rh_gpo, Gpo *rl_gpo): m_gpos(init_gpos(lh_gpo, ll_gpo, rh_gpo, rl_gpo)), m_curr_state(0) {
		apply_state(s_states[m_curr_state]);
	}

	inline StepMotor::~StepMotor() noexcept {
		MotorState shutdown_state {
			{Shoulder::LH, Gpo::State::LOW},
			{Shoulder::LL, Gpo::State::LOW},
			{Shoulder::RH, Gpo::State::LOW},
			{Shoulder::RL, Gpo::State::LOW}
		};
		apply_state(shutdown_state);
	}

	inline void StepMotor::step(const Direction& dir) {
		auto next_state_iter(m_curr_state);
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
		apply_state(s_states[next_state_iter]);
		m_curr_state = next_state_iter;
	}

	inline void StepMotor::apply_state(const MotorState& state) {
		for (auto item: state) {
			m_gpos[item.first]->set_state(item.second);
		}
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

	inline int StepMotor::next_state_cw(int curr_state) {
		const int next_state_iter(curr_state + 1);
		if (MOTOR_STATES_NUMBER <= next_state_iter) {
			return 0;
		}
		return next_state_iter;
	}

	inline int StepMotor::next_state_ccw(int curr_state) {
		if (0 == curr_state) {
			return MOTOR_STATES_NUMBER - 1;
		}
		return curr_state - 1;
	}
}
#endif // STEP_MOTOR_HPP