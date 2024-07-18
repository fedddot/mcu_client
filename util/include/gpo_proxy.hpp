#ifndef	GPO_PROXY_HPP
#define	GPO_PROXY_HPP

#include <memory>
#include <stdexcept>
#include <string>

#include "data.hpp"
#include "gpo.hpp"
#include "gpio.hpp"
#include "integer.hpp"
#include "mcu_client.hpp"
#include "mcu_client_types.hpp"
#include "mcu_task_type.hpp"
#include "object.hpp"
#include "parser.hpp"
#include "serializer.hpp"
#include "string.hpp"

namespace mcu_client_utl {
	class GpoProxy: public mcu_task_engine::Gpo {
	public:
		using McuData = mcu_client::ClientData;
		using McuClient = mcu_client::McuClient<McuData>;
		using McuDataParser = mcu_server::Parser<engine::Data *(const McuData&)>;
		using McuDataSerializer = mcu_server::Serializer<McuData(const engine::Data&)>;
		
		GpoProxy(int id, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer);
		GpoProxy(const GpoProxy& other) = delete;
		GpoProxy& operator=(const GpoProxy& other) = delete;
		~GpoProxy() noexcept override;

		State state() const override;
		void set_state(const State& state) override;
		mcu_task_engine::Gpo *clone() const override;
	private:
		int m_id;
		McuClient *m_client;
		std::unique_ptr<McuDataParser> m_parser;
		std::unique_ptr<McuDataSerializer> m_serializer;

		static void process_report(const engine::Data& report);
	};

	inline GpoProxy::GpoProxy(int id, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer): m_id(id), m_client(client), m_parser(parser.clone()), m_serializer(serializer.clone()) {
		if (!m_client) {
			throw std::invalid_argument("invalid client ptr received");
		}

		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(static_cast<int>(mcu_task_engine::McuTaskType::CREATE_GPIO)));
		request.add("gpio_dir", Integer(static_cast<int>(Direction::OUT)));

		auto serial_ctor_report = m_client->run(m_serializer->serialize(request));
		std::unique_ptr<Data> parsed_report(m_parser->parse(serial_ctor_report));
		process_report(*parsed_report);
	}

	inline GpoProxy::~GpoProxy() noexcept {
		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(static_cast<int>(mcu_task_engine::McuTaskType::DELETE_GPIO)));
		m_client->run(m_serializer->serialize(request));
	}

	inline GpoProxy::State GpoProxy::state() const {
		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(static_cast<int>(mcu_task_engine::McuTaskType::GET_GPIO)));

		auto serial_get_state_report = m_client->run(m_serializer->serialize(request));
		std::unique_ptr<Data> parsed_report(m_parser->parse(serial_get_state_report));
		process_report(*parsed_report);		
		return static_cast<State>(Data::cast<Integer>(Data::cast<Object>(*parsed_report).access("gpio_state")).get());
	}

	inline void GpoProxy::set_state(const State& state) {
		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(static_cast<int>(mcu_task_engine::McuTaskType::SET_GPIO)));
		request.add("gpio_state", Integer(static_cast<int>(state)));

		auto serial_set_state_report = m_client->run(m_serializer->serialize(request));
		std::unique_ptr<Data> parsed_report(m_parser->parse(serial_set_state_report));
		process_report(*parsed_report);		
	}

	inline void GpoProxy::process_report(const engine::Data& report) {
		using namespace engine;
		if (0 != Data::cast<Integer>(Data::cast<Object>(report).access("result")).get()) {
			if (Data::cast<Object>(report).contains("what")) {
				throw std::runtime_error(Data::cast<String>(Data::cast<Object>(report).access("what")).get());
			}
			throw std::runtime_error("server returned a failure report");
		}
	}

	inline mcu_task_engine::Gpo *GpoProxy::clone() const {
		throw std::runtime_error("NOT IMPLEMENTED");
	}
}
#endif // GPO_PROXY_HPP