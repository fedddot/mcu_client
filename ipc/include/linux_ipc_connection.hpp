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
	
	class LinuxIpcConnection: public mcu_ipc::IpcConnection<UartIpcData> {
	public:
		enum class Baud: int {
			BAUD9600,
			BAUD115200
		};
		LinuxIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size);
		LinuxIpcConnection(const LinuxIpcConnection&) = delete;
		LinuxIpcConnection& operator=(const LinuxIpcConnection&) = delete;
		~LinuxIpcConnection() noexcept override;

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

	inline LinuxIpcConnection::LinuxIpcConnection(const std::string& tty_path, const Baud& baud, const UartIpcData& head, const UartIpcData& tail, const std::size_t& max_buff_size):
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
			&LinuxIpcConnection::runner,
			this
		);
	}

	inline LinuxIpcConnection::~LinuxIpcConnection() noexcept {
		m_is_listening.store(false, std::memory_order_acquire);
		m_listening_thread.join();
		uninit_tty(m_fd);
	}

	inline bool LinuxIpcConnection::readable() const {
		return m_connection.readable();
	}

	inline UartIpcData LinuxIpcConnection::read() {
		return m_connection.read();
	}

	inline void LinuxIpcConnection::send(const std::string& data) const {
		m_connection.send(data);
	}
}

#endif // LINUX_UART_IPC_CONNECTION_HPP