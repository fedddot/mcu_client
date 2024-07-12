#ifndef	MCU_CLIENT_HPP
#define	MCU_CLIENT_HPP

#include <stdexcept>

#include "client.hpp"
#include "server_connection.hpp"

namespace mcu_client {
	template <typename Signature>
	class McuClient;

	template <typename Tdata>
	class McuClient<Tdata(const Tdata&)>: public Client<Tdata(const Tdata&)> {
	public:
		McuClient(ServerConnection<Tdata> *connection);
		McuClient(const McuClient& other) = delete;
		McuClient& operator=(const McuClient& other) = delete;

		Tdata run(const Tdata& data) const override;
	private:
		ServerConnection<Tdata> *m_connection;
	};

	template <typename Tdata>
	inline McuClient<Tdata(const Tdata&)>::McuClient(ServerConnection<Tdata> *connection): m_connection(connection) {
		if (!m_connection) {
			throw std::invalid_argument("invalid connection ptr received");
		}
	}

	template <typename Tdata>
	inline Tdata McuClient<Tdata(const Tdata&)>::run(const Tdata& data) const {
		m_connection->send(data);
		return m_connection->read();
	}
}

#endif // MCU_CLIENT_HPP