#ifndef	LINUX_UART_IPC_CONNECTION_HPP
#define	LINUX_UART_IPC_CONNECTION_HPP

#include <atomic>
#include <mutex>
#include <string>
#include <thread>

#include "buffered_custom_ipc_connection.hpp"
#include "ipc_connection.hpp"

namespace linux_mcu_ipc {

	using UartIpcData = std::string;
	
	class UartIpcConnection: public mcu_ipc::IpcConnection<UartIpcData> {
	public:
		enum class Baud: int {
			BAUD9600,
			BAUD115200
		};
		UartIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size);
		UartIpcConnection(const UartIpcConnection&) = delete;
		UartIpcConnection& operator=(const UartIpcConnection&) = delete;
		~UartIpcConnection() noexcept override;

		bool readable() const override;
		UartIpcData read() override;
		void send(const UartIpcData& data) const override;
	private:
		int m_fd;
		std::atomic<bool> m_is_listening;
		std::thread m_listening_thread;
		std::mutex m_mux;

		using CustomConnection = mcu_ipc_utl::BufferedCustomIpcConnection<UartIpcData>;		
		CustomConnection m_connection;

		void runner();
		
		static int init_tty(const std::string& tty_path, const Baud& baud);
		static void uninit_tty(int fd);
		static bool poll_fd(int fd);
		static UartIpcData read_from_fd(int fd);
		static void write_to_fd(int fd, const UartIpcData& data);
	};

	inline UartIpcConnection::UartIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size):
		m_fd(init_tty(tty_path, baud)),
		m_is_listening(false),
		m_connection(
			head,
			tail,
			max_buff_size,
			[this](const UartIpcData& data) {
				write_to_fd(m_fd, data);
			}
		) {
		
		m_is_listening.store(true, std::memory_order_release);
		m_listening_thread = std::thread(
			&UartIpcConnection::runner,
			this
		);
	}

	inline UartIpcConnection::~UartIpcConnection() noexcept {
		m_is_listening.store(false, std::memory_order_acquire);
		m_listening_thread.join();
		uninit_tty(m_fd);
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
}

#endif // LINUX_UART_IPC_CONNECTION_HPP