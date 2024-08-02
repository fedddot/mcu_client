#ifndef	MCU_CLIENT_HPP
#define	MCU_CLIENT_HPP

#include <chrono>
#include <stdexcept>
#include <thread>
#include <unistd.h>

#include "ipc_connection.hpp"

namespace mcu_client {
	template <typename Tdata>
	class McuClient {
	public:
		McuClient(mcu_ipc::IpcConnection<Tdata> *connection);
		McuClient(const McuClient& other) = delete;
		McuClient& operator=(const McuClient& other) = delete;
		virtual ~McuClient() noexcept = default;

		Tdata run(const Tdata& data) const;
	private:
		mcu_ipc::IpcConnection<Tdata> *m_connection;
	};

	template <typename Tdata>
	inline McuClient<Tdata>::McuClient(mcu_ipc::IpcConnection<Tdata> *connection): m_connection(connection) {
		if (!m_connection) {
			throw std::invalid_argument("invalid connection ptr received");
		}
	}

	template <typename Tdata>
	inline Tdata McuClient<Tdata>::run(const Tdata& data) const {
		m_connection->send(data);
		while (!m_connection->readable()) {
			std::this_thread::sleep_for(std::chrono::milliseconds(1));
		}
		return m_connection->read();
	}
}

#endif // MCU_CLIENT_HPP