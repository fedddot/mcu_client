#ifndef	LINUX_UART_IPC_CONNECTION_HPP
#define	LINUX_UART_IPC_CONNECTION_HPP

#include <string>

#include "buffered_custom_ipc_connection.hpp"
#include "ipc_connection.hpp"

namespace linux_mcu_ipc {

	using UartIpcData = std::string;
	
	class UartIpcConnection: public mcu_ipc::IpcConnection<UartIpcData> {
	public:
		enum class Baud: int {
			B9600,
			B115200
		};
		UartIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size);
		UartIpcConnection(const UartIpcConnection&) = delete;
		UartIpcConnection& operator=(const UartIpcConnection&) = delete;
		~UartIpcConnection() noexcept override;

		bool readable() const override;
		UartIpcData read() override;
		void send(const UartIpcData& data) const override;
	private:
		using CustomConnection = mcu_ipc_utl::BufferedCustomIpcConnection<UartIpcData>;
		
		CustomConnection m_connection;

		static CustomConnection *s_connection;
		static void on_received_cb();
		static void send_data(const UartIpcData& data);
		static uint baud_to_uint(const Baud& baud);
	};

	inline UartIpcConnection::UartIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size): m_connection(head, tail, max_buff_size, send_data) {
		if (nullptr != s_connection) {
			throw std::runtime_error("uart connection is already created");
		}
		throw std::runtime_error("NOT IMPLEMENTED");
	}

	inline UartIpcConnection::~UartIpcConnection() noexcept {
		throw std::runtime_error("NOT IMPLEMENTED");
	}

	inline bool UartIpcConnection::readable() const {
		return m_connection.readable();
	}

	inline UartIpcData UartIpcConnection::read() {
		return m_connection.read();
	}

	inline void UartIpcConnection::send(const std::string& data) const {
		m_connection.send(data);
	}

	inline void UartIpcConnection::on_received_cb() {
		throw std::runtime_error("NOT IMPLEMENTED");
	}

	inline void UartIpcConnection::send_data(const UartIpcData& data) {
		throw std::runtime_error("NOT IMPLEMENTED");
	}

	inline uint UartIpcConnection::baud_to_uint(const Baud& baud) {
		switch (baud) {
		case Baud::B9600:
			return 9600;
		case Baud::B115200:
			return 115200;
		default:
			throw std::invalid_argument("unsupported baud");
		}
	}
}

#endif // LINUX_UART_IPC_CONNECTION_HPP