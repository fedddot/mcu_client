#ifndef	PICO_IPC_CONNECTION_HPP
#define	PICO_IPC_CONNECTION_HPP

#include "buffered_custom_ipc_connection.hpp"
#include "hardware/gpio.h"
#include "hardware/irq.h"
#include "hardware/regs/intctrl.h"
#include "hardware/uart.h"
#include "ipc_connection.hpp"
#include <stdexcept>
#include <string>

namespace pico_mcu_ipc {

	using PicoIpcData = std::string;
	
	class PicoIpcConnection: public mcu_ipc::IpcConnection<PicoIpcData> {
	public:
		enum class Baud: int {
			B9600,
			B115200
		};
		PicoIpcConnection(const Baud& baud, const PicoIpcData& head, const PicoIpcData& tail, const std::size_t& max_buff_size);
		PicoIpcConnection(const PicoIpcConnection&) = delete;
		PicoIpcConnection& operator=(const PicoIpcConnection&) = delete;
		~PicoIpcConnection() noexcept override;

		bool readable() const override;
		PicoIpcData read() override;
		void send(const PicoIpcData& data) const override;
	private:
		enum : uint {
			UART0_TX_PIN = 0,
			UART0_RX_PIN = 1,
			DATA_BITS = 8,
			STOP_BITS = 1
		};
		using CustomConnection = mcu_ipc_utl::BufferedCustomIpcConnection<PicoIpcData>;
		
		CustomConnection m_connection;

		static CustomConnection *s_connection;
		static void on_received_cb();
		static void send_data(const PicoIpcData& data);
		static uint baud_to_uint(const Baud& baud);
	};

	inline PicoIpcConnection::PicoIpcConnection(const Baud& baud, const PicoIpcData& head, const PicoIpcData& tail, const std::size_t& max_buff_size): m_connection(head, tail, max_buff_size, send_data) {
		if (nullptr != s_connection) {
			throw std::runtime_error("uart0 connection is already created");
		}
		uart_init(uart0, baud_to_uint(baud));
		gpio_set_function(UART0_TX_PIN, GPIO_FUNC_UART);
    	gpio_set_function(UART0_RX_PIN, GPIO_FUNC_UART);
		uart_set_baudrate(uart0, baud_to_uint(baud));
		uart_set_hw_flow(uart0, false, false);
		uart_set_format(uart0, DATA_BITS, STOP_BITS, UART_PARITY_NONE);
		uart_set_fifo_enabled(uart0, false);
	    irq_set_exclusive_handler(UART0_IRQ, &on_received_cb);
    	irq_set_enabled(UART0_IRQ, true);
    	uart_set_irq_enables(uart0, true, false);
		s_connection = &m_connection;
	}

	inline PicoIpcConnection::~PicoIpcConnection() noexcept {
		irq_set_enabled(UART0_IRQ, false);
    	uart_set_irq_enables(uart0, false, false);
    	uart_deinit(uart0);
		gpio_set_function(UART0_TX_PIN, GPIO_FUNC_NULL);
    	gpio_set_function(UART0_RX_PIN, GPIO_FUNC_NULL);
		s_connection = nullptr;
	}

	inline bool PicoIpcConnection::readable() const {
		return m_connection.readable();
	}

	inline PicoIpcData PicoIpcConnection::read() {
		return m_connection.read();
	}

	inline void PicoIpcConnection::send(const std::string& data) const {
		m_connection.send(data);
	}

	inline void PicoIpcConnection::on_received_cb() {
		PicoIpcData data("");
		while (uart_is_readable(uart0)) {
			data.push_back(uart_getc(uart0));
		}
		if (!s_connection) {
			return;
		}
		s_connection->feed(data);
		irq_clear(UART0_IRQ);
	}

	inline void PicoIpcConnection::send_data(const PicoIpcData& data) {
		for (auto ch: data) {
			uart_putc(uart0, ch);
		}
	}

	inline uint PicoIpcConnection::baud_to_uint(const Baud& baud) {
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

#endif // PICO_IPC_CONNECTION_HPP