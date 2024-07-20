#ifndef	GPIO_DATA_GENERATOR_HPP
#define	GPIO_DATA_GENERATOR_HPP

#include <string>

#include "array.hpp"
#include "gpio.hpp"
#include "integer.hpp"
#include "mcu_task_type.hpp"
#include "object.hpp"

namespace mcu_client_utl {
	class GpioDataGenerator {
	public:
		using GpioDirection = typename mcu_task_engine::Gpio::Direction;
		using GpioState = typename mcu_task_engine::Gpio::State;

		GpioDataGenerator() = default;
		GpioDataGenerator(const GpioDataGenerator&) = default;
		GpioDataGenerator& operator=(const GpioDataGenerator&) = default;

		engine::Object create_gpio_data(int id, const GpioDirection& dir) const;
		engine::Object delete_gpio_data(int id) const;
		engine::Object set_gpio_data(int id, const GpioState& state) const;
		engine::Object delay_data(int delay_ms) const;
		engine::Object sequence_data(const engine::Array& tasks) const;
	};

	inline engine::Object GpioDataGenerator::create_gpio_data(int id, const GpioDirection& dir) const {
		engine::Object data;
		data.add("ctor_id", engine::Integer(static_cast<int>(mcu_task_engine::McuTaskType::CREATE_GPIO)));
		data.add("gpio_id", engine::Integer(id));
		data.add("gpio_dir", engine::Integer(static_cast<int>(dir)));
		return data;
	}

	inline engine::Object GpioDataGenerator::delete_gpio_data(int id) const {
		engine::Object data;
		data.add("ctor_id", engine::Integer(static_cast<int>(mcu_task_engine::McuTaskType::DELETE_GPIO)));
		data.add("gpio_id", engine::Integer(id));
		return data;
	}

	inline engine::Object GpioDataGenerator::set_gpio_data(int id, const GpioState& state) const {
		engine::Object data;
		data.add("ctor_id", engine::Integer(static_cast<int>(mcu_task_engine::McuTaskType::SET_GPIO)));
		data.add("gpio_id", engine::Integer(id));
		data.add("gpio_state", engine::Integer(static_cast<int>(state)));
		return data;
	}

	inline engine::Object GpioDataGenerator::delay_data(int delay_ms) const {
		engine::Object data;
		data.add("ctor_id", engine::Integer(static_cast<int>(mcu_task_engine::McuTaskType::DELAY)));
		data.add("delay_ms", engine::Integer(delay_ms));
		return data;
	}

	inline engine::Object GpioDataGenerator::sequence_data(const engine::Array& tasks) const {
		engine::Object data;
		data.add("ctor_id", engine::Integer(static_cast<int>(mcu_task_engine::McuTaskType::SEQUENCE)));
		data.add("tasks", tasks);
		return data;
	}
}
#endif // GPIO_DATA_GENERATOR_HPP