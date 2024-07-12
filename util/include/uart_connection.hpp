#ifndef	UART_CONNECTION_HPP
#define	UART_CONNECTION_HPP

#include "custom_listener.hpp"
#include "custom_receiver.hpp"
#include "listener.hpp"
#include "mcu_client_types.hpp"
#include "server_connection.hpp"
#include "uart.hpp"
#include <atomic>
#include <memory>
#include <mutex>
#include <stdexcept>
#include <string>
#include <thread>

namespace mcu_client_utl {

	class UartConnection: public mcu_client::ServerConnection<mcu_client::ClientData> {
	public:
		using UartData = mcu_client::ClientData;
		UartConnection(Uart *uart, unsigned int timeout_ms, const UartData& head, const UartData& tail);
		UartConnection(const UartConnection& other) = delete;
		UartConnection& operator=(const UartConnection& other) = delete;

		void send(const mcu_client::ClientData&) const override;
		mcu_client::ClientData read() const override;
	private:
		Uart *m_uart;
		unsigned int m_timeout_ms;
		UartData m_head;
		UartData m_tail;
		mcu_server_utl::CustomReceiver m_receiver;
	};

	inline UartConnection::UartConnection(Uart *uart, unsigned int timeout_ms, const UartData& head, const UartData& tail): m_uart(uart), m_timeout_ms(timeout_ms), m_head(head), m_tail(tail), m_receiver(head, tail) {
		if (!m_uart) {
			throw std::invalid_argument("invalid uart ptr received");
		}
	}

	inline void UartConnection::send(const mcu_client::ClientData& data) const {
		m_uart->start_listening(
			mcu_server_utl::CustomListener<UartData>(
				[](const UartData& data) {

				}
			)
		);
		m_uart->send(m_head + data + m_tail);
	}

	inline mcu_client::ClientData UartConnection::read() const {

	const mcu_client::ClientData test_data("MSG_HEADER{\"ctor_id\" : 0, \"gpio_dir\" : 1, \"gpio_id\" : 25}MSG_TAIL");
	bool data_received(false);
	std::mutex mux;
	std::condition_variable cond;

	CustomListener<mcu_client::ClientData> test_listener(
		[&data_received, &mux, &cond](const mcu_client::ClientData& data) {
			std::unique_lock lock(mux);
			std::cout << "data received: " << data << std::endl;
			data_received = true;
			cond.notify_one();
			std::cout << "notified" << std::endl;
		}
	);
	
	// WHEN
	Uart instance("/dev/ttyACM0", Uart::UartBaud::BAUD9600, 100);
	mcu_client::ClientData result("");

	// THEN
	ASSERT_NO_THROW(instance.start_listening(test_listener));
	ASSERT_TRUE(instance.is_listening());
	
	std::unique_lock lock(mux);
	ASSERT_NO_THROW(instance.send(test_data));
	
	cond.wait(lock);
	ASSERT_NO_THROW(instance.stop_listening());
	ASSERT_FALSE(instance.is_listening());
	}
}

#endif // UART_CONNECTION_HPP