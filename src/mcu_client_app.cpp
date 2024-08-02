#include <algorithm>
#include <exception>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

#include "linux_ipc_connection.hpp"
#include "mcu_client.hpp"

#ifndef MSG_HEADER
#   define MSG_HEADER "MSG_HEADER"
#endif

#ifndef MSG_TAIL
#   define MSG_TAIL "MSG_TAIL"
#endif

using namespace mcu_client;
using namespace linux_mcu_ipc;

using ClientData = UartIpcData;

class ClientConfig {
public:
    ClientConfig(int argc, char **argv);
    std::string tty_path() const;
    LinuxIpcConnection::Baud baud() const;
    unsigned int read_timeout_ms() const;

    ClientData read_data() const;
    void write_result(const ClientData& result) const;

    void log_info(const std::string& what) const;
    void log_error(const std::string& what) const;
private:
    std::vector<std::string> m_values;
    std::string get_param(const std::string& flag) const;
};

int main(int argc, char **argv) {
    ClientConfig cfg(argc, argv);
    try {
        LinuxIpcConnection connection(
            cfg.tty_path(),
            cfg.baud(),
            MSG_HEADER,
            MSG_TAIL,
            1000UL
        );
        cfg.log_info("MCU server connection created");
        
        McuClient<ClientData> client(&connection);
        cfg.log_info("MCU client created");

        auto data = cfg.read_data();
        cfg.log_info("sending data:\n" + data);
        
        auto report = client.run(data);
        cfg.log_info("received report:\n" + report);

        cfg.write_result(report);
    } catch (const std::exception& e) {
        cfg.log_error("an exception caught while client execution: " + std::string(e.what()));
        return -1;
    }
    return 0;
}

inline ClientConfig::ClientConfig(int argc, char **argv) {
    for (auto i = 0; i < argc; ++i) {
        m_values.push_back(std::string(argv[i]));
    }
}
    
inline std::string ClientConfig::tty_path() const {
    return get_param("-d");
}

inline LinuxIpcConnection::Baud ClientConfig::baud() const {
    auto baud_str = get_param("-b");
    if ("9600" == baud_str) {
        return LinuxIpcConnection::Baud::BAUD9600;
    } else if ("115200" == baud_str) {
        return LinuxIpcConnection::Baud::BAUD115200;
    }
    throw std::runtime_error("unsupported baud rate received");
}

inline unsigned int ClientConfig::read_timeout_ms() const {
    return std::stoi(get_param("-t"));
}

inline ClientData ClientConfig::read_data() const {
    auto file_path = get_param("-i");
    std::ifstream file(file_path);
    if (!file.is_open()) {
        throw std::runtime_error("failed to open " + file_path);
    }
    ClientData content("");
    log_info("reading data from " + file_path);
    while (true) {
        ClientData line("");
        if (!std::getline(file, line)) {
            break;
        }
        line.insert(line.end(), '\n');
        content.insert(content.end(), line.begin(), line.end());
    }
    file.close();
    return content;
}

inline void ClientConfig::write_result(const ClientData& result) const {
    auto file_path = get_param("-o");
    std::ofstream file(file_path);
    if (!file.is_open()) {
        throw std::runtime_error("failed to open " + file_path);
    }
    file << result;
    file.close();
}

inline void ClientConfig::log_info(const std::string& what) const {
    std::cout << "[  INFO ]: " << what << std::endl;
}
inline void ClientConfig::log_error(const std::string& what) const {
    std::cout << "[ ERROR ]: " << what << std::endl;
}

inline std::string ClientConfig::get_param(const std::string& flag) const {
    auto flag_iter = std::find(m_values.begin(), m_values.end(), flag);
    if (m_values.end() == flag_iter) {
        throw std::runtime_error("flag " + flag + " was not received");
    }
    auto val_iter = flag_iter + 1;
    if (m_values.end() == val_iter) {
        throw std::runtime_error("value for " + flag + " was not found");
    }
    return *val_iter;
}