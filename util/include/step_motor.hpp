#ifndef	STEP_MOTOR_HPP
#define	STEP_MOTOR_HPP

#include <array>
#include <iostream>
#include <map>
#include <memory>
#include <stdexcept>
#include <string>

#include "array.hpp"
#include "data.hpp"
#include "gpio.hpp"
#include "gpio_data_generator.hpp"
#include "integer.hpp"
#include "mcu_client.hpp"
#include "mcu_client_types.hpp"
#include "object.hpp"
#include "parser.hpp"
#include "serializer.hpp"
#include "string.hpp"

namespace mcu_client_utl {
	class StepMotor {
	public:
		enum class Direction: int {
			CW,
			CCW
		};
		using McuData = mcu_client::ClientData;
		using McuClient = mcu_client::McuClient<McuData>;
		using McuDataParser = mcu_server::Parser<engine::Data *(const McuData&)>;
		using McuDataSerializer = mcu_server::Serializer<McuData(const engine::Data&)>;

		StepMotor(int lh_gpo, int ll_gpo, int rh_gpo, int rl_gpo, int step_duration, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer);
		StepMotor(const StepMotor& other) = delete;
		StepMotor& operator=(const StepMotor& other) = delete;
		~StepMotor() noexcept;

		void steps(unsigned int num, const Direction& dir);
	private:
		enum class Shoulder: int {
			LH,
			LL,
			RH,
			RL
		};
		enum: int { MOTOR_STATES_NUMBER = 8 };

		using GpioState = typename mcu_task_engine::Gpio::State;
		using GpioDirection = typename mcu_task_engine::Gpio::Direction;
		using MotorState = std::map<Shoulder, GpioState>;
		using MotorStates = std::array<MotorState, MOTOR_STATES_NUMBER>;
		using MotorGpos = std::map<Shoulder, int>;

		mutable MotorGpos m_gpos;
		int m_step_duration;
		McuClient *m_client;
		std::unique_ptr<McuDataParser> m_parser;
		std::unique_ptr<McuDataSerializer> m_serializer;
		
		int m_state_index;

		void create_gpios() const;
		void delete_gpios() const;
		engine::Array generate_step_tasks(const MotorState& state) const;
		void run_client(const engine::Data& data) const;
		static const MotorStates s_states;
		static MotorGpos init_gpos(int lh_gpo, int ll_gpo, int rh_gpo, int rl_gpo);
		static int next_state_cw(int curr_state);
		static int next_state_ccw(int curr_state);
		static engine::Data *generate_state_application_data(const MotorState& state);
	};

	inline void StepMotor::create_gpios() const {
		using namespace engine;
		Array create_tasks;
		GpioDataGenerator data_generator;
		for (auto item: m_gpos) {
			create_tasks.push_back(data_generator.create_gpio_data(item.second, GpioDirection::OUT));
		}
		auto sequence = data_generator.sequence_data(create_tasks);
		run_client(sequence);
	}

	inline void StepMotor::delete_gpios() const {
		using namespace engine;
		Array delete_tasks;
		GpioDataGenerator data_generator;
		for (auto item: m_gpos) {
			delete_tasks.push_back(data_generator.delete_gpio_data(item.second));
		}
		auto sequence = data_generator.sequence_data(delete_tasks);
		run_client(sequence);
	}

	inline engine::Array StepMotor::generate_step_tasks(const MotorState& state) const {
		using namespace engine;
		Array tasks;
		GpioDataGenerator data_generator;
		for (auto item: state) {
			tasks.push_back(data_generator.set_gpio_data(m_gpos[item.first], item.second));
		}
		tasks.push_back(data_generator.delay_data(m_step_duration));
		return tasks;
	}

	inline void StepMotor::run_client(const engine::Data& data) const {
		auto raw_serial_data = m_serializer->serialize(data);
		McuData serial_data("");
		for (auto ch: raw_serial_data) {
			if (('\n' == ch) || ('\t' == ch) || (' ' == ch)) {
				continue;
			}
			serial_data.push_back(ch);
		}
		std::cout << std::endl << "sending data:" << std::endl << serial_data << std::endl;
		std::cout << "data size = " << std::to_string(serial_data.size()) << std::endl;
		auto report = m_client->run(serial_data);
		std::cout << "report:" << std::endl << report << std::endl;
		std::unique_ptr<engine::Data> parsed_report(m_parser->parse(report));
		auto result = engine::Data::cast<engine::Integer>(engine::Data::cast<engine::Object>(*parsed_report).access("result")).get();
		if (0 != result) {
			throw std::runtime_error(engine::Data::cast<engine::String>(engine::Data::cast<engine::Object>(*parsed_report).access("what")).get());
		}
	}

	inline StepMotor::StepMotor(int lh_gpo, int ll_gpo, int rh_gpo, int rl_gpo, int step_duration, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer): m_gpos(init_gpos(lh_gpo, ll_gpo, rh_gpo, rl_gpo)), m_step_duration(step_duration), m_client(client), m_parser(parser.clone()), m_serializer(serializer.clone()), m_state_index(0) {
		create_gpios();
		auto tasks = generate_step_tasks(s_states[m_state_index]);
		GpioDataGenerator data_generator;
		auto sequence_task = data_generator.sequence_data(tasks);
		run_client(sequence_task);
	}

	inline StepMotor::~StepMotor() noexcept {
		MotorState shutdown_state {
			{Shoulder::LH, GpioState::LOW},
			{Shoulder::LL, GpioState::LOW},
			{Shoulder::RH, GpioState::LOW},
			{Shoulder::RL, GpioState::LOW}
		};
		auto tasks = generate_step_tasks(shutdown_state);
		GpioDataGenerator data_generator;
		auto sequence_task = data_generator.sequence_data(tasks);
		run_client(sequence_task);
		delete_gpios();
	}

	inline void StepMotor::steps(unsigned int num, const Direction& dir) {
		using namespace engine;
		Array tasks;
		while (num) {
			auto next_state_index = m_state_index;
			switch (dir) {
			case Direction::CW:
				next_state_index = next_state_cw(m_state_index);
				break;
			case Direction::CCW:
				next_state_index = next_state_ccw(m_state_index);
				break;
			default:
				throw std::invalid_argument("unsupported direction received");
			}
			generate_step_tasks(s_states[next_state_index]).for_each(
				[&tasks](int index, const Data& data) {
					tasks.push_back(data);
				}
			);
			m_state_index = next_state_index;
			--num;
		}
		GpioDataGenerator data_generator;
		run_client(data_generator.sequence_data(tasks));
	}

	inline StepMotor::MotorGpos StepMotor::init_gpos(int lh_gpo, int ll_gpo, int rh_gpo, int rl_gpo) {
		return MotorGpos {
			{Shoulder::LH, lh_gpo},
			{Shoulder::LL, ll_gpo},
			{Shoulder::RH, rh_gpo},
			{Shoulder::RL, rl_gpo}
		};
	}

	inline int StepMotor::next_state_cw(int curr_state) {
		const int next_state_iter(curr_state + 1);
		if (MOTOR_STATES_NUMBER <= next_state_iter) {
			return 0;
		}
		return next_state_iter;
	}

	inline int StepMotor::next_state_ccw(int curr_state) {
		const int next_state_iter(curr_state - 1);
		if (0 > next_state_iter) {
			return MOTOR_STATES_NUMBER - 1;
		}
		return next_state_iter;
	}
}
#endif // STEP_MOTOR_HPP