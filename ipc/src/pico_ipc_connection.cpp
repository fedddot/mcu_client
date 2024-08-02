#include "pico_ipc_connection.hpp"

mcu_ipc_utl::BufferedCustomIpcConnection<pico_mcu_ipc::PicoIpcData> *pico_mcu_ipc::PicoIpcConnection::s_connection(nullptr);