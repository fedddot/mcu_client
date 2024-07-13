#ifndef	UART_CONNECTION_HPP
#define	UART_CONNECTION_HPP

#include <chrono>
#include <condition_variable>
#include <mutex>
#include <stdexcept>
#include <string>

#include "custom_listener.hpp"
#include "custom_receiver.hpp"
#include "mcu_client_types.hpp"
#include "server_connection.hpp"
#include "uart.hpp"

namespace mcu_client_utl {

	class UartConnection: public mcu_client::ServerConnection<mcu_client::ClientData> {
	public:
		using UartData = mcu_client::ClientData;
		UartConnection(Uart *uart, unsigned int timeout_ms, const UartData& head, const UartData& tail);
		UartConnection(const UartConnection& other) = delete;
		UartConnection& operator=(const UartConnection& other) = delete;

		void send(const UartData&) const override;
		UartData read() const override;
	private:
		Uart *m_uart;
		unsigned int m_timeout_ms;
		UartData m_head;
		UartData m_tail;

		mutable mcu_server_utl::CustomReceiver m_receiver;
		mutable std::mutex m_mux;
		mutable std::condition_variable m_cond;
		mutable UartData m_received_data;
		mutable bool m_is_data_received;
	};

	inline UartConnection::UartConnection(Uart *uart, unsigned int timeout_ms, const UartData& head, const UartData& tail): m_uart(uart), m_timeout_ms(timeout_ms), m_head(head), m_tail(tail), m_receiver(head, tail), m_received_data(""), m_is_data_received(false) {
		if (!m_uart) {
			throw std::invalid_argument("invalid uart ptr received");
		}
		m_receiver.set_listener(
			mcu_server_utl::CustomListener<UartData>(
				[this](const UartData& data) {
					std::unique_lock lock(m_mux);
					m_received_data = data;
					m_is_data_received = true;
					m_cond.notify_one();
				}
			)
		);
	}

	inline void UartConnection::send(const UartData& data) const {
		std::lock_guard lock(m_mux);
		m_uart->start_listening(
			mcu_server_utl::CustomListener<UartData>(
				[this](const UartData& data) {
					m_receiver.feed(data);
				}
			)
		);
		m_uart->send(m_head + data + m_tail);
	}

	inline UartConnection::UartData UartConnection::read() const {
		std::unique_lock lock(m_mux);
		if (!m_uart->is_listening()) {
			throw std::runtime_error("send function should be called prior to read");
		}
		if (m_is_data_received) {
			m_uart->stop_listening();
			return m_received_data;
		}
		auto wait_result = m_cond.wait_for(lock, std::chrono::milliseconds(m_timeout_ms));
		m_uart->stop_listening();
		if (std::cv_status::timeout == wait_result) {
			throw std::runtime_error("timeout waiting for server response exceeded");
		}
		auto result = m_received_data;
		m_is_data_received = false;
		m_received_data = "";
		return result;
	}
}

#endif // UART_CONNECTION_HPP