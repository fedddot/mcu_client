#ifndef	SERVER_CONNECTION_HPP
#define	SERVER_CONNECTION_HPP

namespace mcu_client {
	
	template <typename Tdata>
	class ServerConnection {
	public:
		virtual ~ServerConnection() noexcept = default;
		virtual void send(const Tdata&) const = 0;
		virtual Tdata read() const = 0;
	};
}

#endif // SERVER_CONNECTION_HPP