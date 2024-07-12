#ifndef	UART_HPP
#define	UART_HPP

#include "listener.hpp"
#include "mcu_client_types.hpp"
#include <atomic>
#include <memory>
#include <mutex>
#include <stdexcept>
#include <string>
#include <thread>

namespace mcu_client_utl {

	class Uart {
	public:
		enum class UartBaud: int {
			BAUD9600,
			BAUD115200
		};
		Uart(const std::string& tty_path, const UartBaud& baud, unsigned int poll_timeout_ms);
		Uart(const Uart& other) = delete;
		Uart& operator=(const Uart& other) = delete;
				
		~Uart() noexcept;
		void send(const mcu_client::ClientData& data) const;

		void start_listening(const mcu_server::Listener<mcu_client::ClientData>& listener);
		void stop_listening();
		bool is_listening() const;
	private:
		std::string m_tty_path;
		UartBaud m_baud;
		unsigned int m_poll_timeout_ms;
		int m_fd;

		using Listener = mcu_server::Listener<mcu_client::ClientData>;
		std::unique_ptr<Listener> m_listener;
		std::atomic<bool> m_is_listening;
		std::thread m_listening_thread;
		std::mutex m_mux;

		void init_tty();
		void uninit_tty();
		bool poll_fd() const;
		mcu_client::ClientData read_from_fd() const;
		void runner();
	};

	inline Uart::Uart(const std::string& tty_path, const UartBaud& baud, unsigned int read_timeout_ms): m_tty_path(tty_path), m_baud(baud), m_poll_timeout_ms(read_timeout_ms), m_fd(-1), m_is_listening(false) {
		init_tty();
	}

	inline Uart::~Uart() noexcept {
		uninit_tty();
	}

	inline void Uart::start_listening(const mcu_server::Listener<mcu_client::ClientData>& listener) {
		std::lock_guard lock(m_mux);
		if (is_listening()) {
			throw std::runtime_error("uart is already listening");
		}
		m_listener = std::unique_ptr<Listener>(listener.clone());
		m_is_listening.store(true, std::memory_order_release);
		m_listening_thread = std::thread(&Uart::runner, this);
	}

	inline void Uart::stop_listening() {
		std::lock_guard lock(m_mux);
		if (!is_listening()) {
			throw std::runtime_error("uart is not listening");
		}
		m_is_listening.store(false, std::memory_order_release);
		m_listening_thread.join();
		m_listener = nullptr;
	}

	inline bool Uart::is_listening() const {
		return m_is_listening.load(std::memory_order_acquire);
	}
}

#endif // UART_HPP