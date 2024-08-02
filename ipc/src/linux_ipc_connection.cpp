#include <fcntl.h>
#include <stdexcept>
#include <string>
#include <sys/poll.h>
#include <termios.h>
#include <unistd.h>

#include "linux_ipc_connection.hpp"

using namespace linux_mcu_ipc;

void LinuxIpcConnection::runner() {
    while (m_is_listening.load(std::memory_order_acquire)) {
		if (!poll_fd(m_fd)) {
			continue;
		}
		m_connection.feed(read_from_fd(m_fd));
	}
}

int LinuxIpcConnection::init_tty(const std::string& tty_path, const Baud& baud) {
	auto fd = open(tty_path.c_str(), O_RDWR);
	if (0 > fd) {
		throw std::runtime_error("failed to open " + tty_path);
	}
	auto close_and_throw = [fd](const std::exception& e) {
		close(fd);
		throw e;
	};
	auto cast_baud = [close_and_throw](const Baud& baud) {
		switch (baud) {
		case Baud::BAUD9600:
			return B9600;
		case Baud::BAUD115200:
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
	tty.c_oflag &= ~OPOST;			// Prevent special interpretation of output bytes
                                    // (e.g. newline chars)
	tty.c_oflag &= ~ONLCR;			// Prevent conversion of newline to carriage return/line feed

	tty.c_cc[VTIME] = 10;           // Wait for up to m_poll_timeout_ms deciseconds,
                                    // returning as soon as any data is received.
	tty.c_cc[VMIN] = 0;

	cfsetispeed(&tty, cast_baud(baud));
	cfsetospeed(&tty, cast_baud(baud));

	if (tcsetattr(fd, TCSANOW, &tty)) {
		close_and_throw(std::runtime_error("failed to write configuration into " + tty_path));
	}
    return fd;
}

void LinuxIpcConnection::uninit_tty(int fd) {
	close(fd);
}

bool LinuxIpcConnection::poll_fd(int fd) {
    enum: int { POLLING_TIMEOUT_MS = 100 };
	pollfd fd_poller {
		.fd = fd,
		.events = POLLIN | POLLERR | POLLMSG,
		.revents = 0
	};
	auto poll_result = poll(&fd_poller, 1, POLLING_TIMEOUT_MS);
	if (0 > poll_result) {
		throw std::runtime_error("failed to poll fd = " + std::to_string(fd));
	}
	return (0 != poll_result);
}

inline static UartIpcData read_from_fd_wrapper(int fd) {
	UartIpcData data("");
	while (true) {
		enum {READ_BUFF_SIZE = 100};
		char buff[READ_BUFF_SIZE] = {'\0'};
		auto read_size = read(fd, buff, READ_BUFF_SIZE - 1);
		if (0 >= read_size) {
			break;
		}
		UartIpcData additional_data(buff);
		data.insert(data.end(), additional_data.begin(), additional_data.end());
	}
	return data;
}

UartIpcData LinuxIpcConnection::read_from_fd(int fd) {
	return read_from_fd_wrapper(fd);
};

void LinuxIpcConnection::write_to_fd(int fd, const UartIpcData& data) {
	auto write_size = write(fd, data.c_str(), data.size());
	if (write_size != data.size()) {
		throw std::runtime_error("failed to write to fd = " + std::to_string(fd));
	}
}



