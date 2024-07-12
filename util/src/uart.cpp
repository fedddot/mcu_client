#include <atomic>
#include <exception>
#include <fcntl.h>
#include <poll.h>
#include <stdexcept>
#include <string>
#include <sys/poll.h>
#include <termios.h>
#include <unistd.h>

#include "uart.hpp"

using namespace mcu_client_utl;

void Uart::send(const mcu_client::ClientData& data) const {
	auto write_size = write(m_fd, data.c_str(), data.size());
	if (write_size != data.size()) {
		throw std::runtime_error("failed to write to " + m_tty_path);
	}
}

void Uart::runner() {
	while (m_is_listening.load(std::memory_order_acquire)) {
		if (!poll_fd()) {
			continue;
		}
		m_listener->on_event(read_from_fd());
	}
}

void Uart::init_tty() {
	auto fd = open(m_tty_path.c_str(), O_RDWR);
	if (0 > fd) {
		throw std::runtime_error("failed to open " + m_tty_path);
	}
	auto close_and_throw = [fd](const std::exception& e) {
		close(fd);
		throw e;
	};
	auto cast_baud = [close_and_throw](const Uart::UartBaud& baud) {
		switch (baud) {
		case Uart::UartBaud::BAUD9600:
			return B9600;
		case Uart::UartBaud::BAUD115200:
			return B115200;
		default:
			close_and_throw(std::invalid_argument("unsupported BAUD received"));
		}
	};

	termios tty;
	if (tcgetattr(fd, &tty)) {
		close_and_throw(std::runtime_error("failed to retrieve UART config"));
	}
	tty.c_cflag &= ~PARENB;			// parity: none
	tty.c_cflag &= ~CSTOPB;			// one stop-bit

	tty.c_cflag &= ~CSIZE;			// clear all size bits
	tty.c_cflag |= CS8;				// 8 bits per byte

	tty.c_cflag &= ~CRTSCTS;		// Disable RTS/CTS hardware flow control
	tty.c_cflag |= CREAD | CLOCAL; 	// Turn on READ & ignore ctrl lines
	tty.c_lflag &= ~ICANON;			// Disable canonical line-by-line sending

	tty.c_lflag &= ~ECHO;			// Disable echo
	tty.c_lflag &= ~ECHOE;			// Disable erasure
	tty.c_lflag &= ~ECHONL;			// Disable new-line echo

	tty.c_lflag &= ~ISIG; 			// Disable interpretation of INTR, QUIT and SUSP
	tty.c_iflag &= ~(IXON | IXOFF | IXANY); // Turn off s/w flow ctrl
	tty.c_iflag &= ~(IGNBRK|BRKINT|PARMRK|ISTRIP|INLCR|IGNCR|ICRNL); // Disable any special handling of received bytes
	tty.c_oflag &= ~OPOST;			// Prevent special interpretation of output bytes (e.g. newline chars)
	tty.c_oflag &= ~ONLCR;			// Prevent conversion of newline to carriage return/line feed

	tty.c_cc[VTIME] = m_poll_timeout_ms / 100; // Wait for up to m_poll_timeout_ms deciseconds, returning as soon as any data is received.
	tty.c_cc[VMIN] = 0;

	cfsetispeed(&tty, cast_baud(m_baud));
	cfsetospeed(&tty, cast_baud(m_baud));

	if (tcsetattr(fd, TCSANOW, &tty)) {
		close_and_throw(std::runtime_error("failed to write configuration into " + m_tty_path));
	}

	m_fd = fd;
}

void Uart::uninit_tty() {
	close(m_fd);
	m_fd = -1;
}

bool Uart::poll_fd() const {
	pollfd fd_poller {
		.fd = m_fd,
		.events = POLLIN | POLLERR | POLLMSG,
		.revents = 0
	};
	auto poll_result = poll(&fd_poller, 1, m_poll_timeout_ms);
	if (0 > poll_result) {
		throw std::runtime_error("failed to poll fd");
	}
	return (0 != poll_result);
}

mcu_client::ClientData Uart::read_from_fd() const {
	mcu_client::ClientData data("");
	while (true) {
		enum {READ_BUFF_SIZE = 100};
		char buff[READ_BUFF_SIZE] = {'\0'};
		auto read_size = read(m_fd, buff, READ_BUFF_SIZE - 1);
		if (0 >= read_size) {
			break;
		}
		mcu_client::ClientData additional_data(buff);
		data.insert(data.end(), additional_data.begin(), additional_data.end());
	}
	return data;
};