#ifndef	GPI_PROXY_HPP
#define	GPI_PROXY_HPP

#include <memory>
#include <stdexcept>
#include <string>

#include "client.hpp"
#include "data.hpp"
#include "gpi.hpp"
#include "gpio.hpp"
#include "integer.hpp"
#include "mcu_client.hpp"
#include "mcu_client_types.hpp"
#include "object.hpp"
#include "parser.hpp"
#include "serializer.hpp"
#include "string.hpp"

namespace mcu_client_utl {
	class GpiProxy: public mcu_task_engine::Gpi {
	public:
		using McuData = mcu_client::ClientData;
		using McuClient = mcu_client::McuClient<McuData>;
		using McuDataParser = mcu_server::Parser<engine::Data *(const McuData&)>;
		using McuDataSerializer = mcu_server::Serializer<McuData(const engine::Data&)>;
		
		GpiProxy(int id, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer);
		GpiProxy(const GpiProxy& other) = delete;
		GpiProxy& operator=(const GpiProxy& other) = delete;
		~GpiProxy() noexcept override;

		State state() const override;
		mcu_task_engine::Gpio *clone() const override;
	private:
		int m_id;
		McuClient *m_client;
		std::unique_ptr<McuDataParser> m_parser;
		std::unique_ptr<McuDataSerializer> m_serializer;

		static void process_report(const engine::Data& report);
	};

	inline GpiProxy::GpiProxy(int id, McuClient *client, const McuDataParser& parser, const McuDataSerializer& serializer): m_id(id), m_client(client), m_parser(parser.clone()), m_serializer(serializer.clone()) {
		if (!m_client) {
			throw std::invalid_argument("invalid client ptr received");
		}

		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(0));
		request.add("gpio_dir", Integer(0));

		auto serial_ctor_report = m_client->run(m_serializer->serialize(request));
		std::unique_ptr<Data> parsed_report(m_parser->parse(serial_ctor_report));
		process_report(*parsed_report);
	}

	inline GpiProxy::~GpiProxy() noexcept {
		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(1));
		m_client->run(m_serializer->serialize(request));
	}

	inline GpiProxy::State GpiProxy::state() const {
		using namespace engine;
		Object request;
		request.add("gpio_id", Integer(m_id));
		request.add("ctor_id", Integer(3));

		auto serial_ctor_report = m_client->run(m_serializer->serialize(request));
		std::unique_ptr<Data> parsed_report(m_parser->parse(serial_ctor_report));
		process_report(*parsed_report);		
		return static_cast<State>(Data::cast<Integer>(Data::cast<Object>(*parsed_report).access("gpio_state")).get());
	}

	inline void GpiProxy::process_report(const engine::Data& report) {
		using namespace engine;
		if (0 != Data::cast<Integer>(Data::cast<Object>(report).access("result")).get()) {
			if (Data::cast<Object>(report).contains("what")) {
				throw std::runtime_error(Data::cast<String>(Data::cast<Object>(report).access("what")).get());
			}
			throw std::runtime_error("server returned a failure report");
		}
	}

	inline mcu_task_engine::Gpio *GpiProxy::clone() const {
		throw std::runtime_error("NOT IMPLEMENTED");
	}
}
#endif // GPI_PROXY_HPP