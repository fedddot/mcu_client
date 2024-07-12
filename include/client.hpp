#ifndef	CLIENT_HPP
#define	CLIENT_HPP

namespace mcu_client {
	template <typename Signature>
	class Client;

	template <typename Tprod, typename ...Args>
	class Client<Tprod(Args...)> {
	public:
		virtual ~Client() noexcept = default;
		virtual Tprod run(Args...) const = 0;
	};
}

#endif // CLIENT_HPP