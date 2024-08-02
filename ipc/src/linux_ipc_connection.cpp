#include "linux_ipc_connection.hpp"
#include "buffered_custom_ipc_connection.hpp"

mcu_ipc_utl::BufferedCustomIpcConnection<linux_mcu_ipc::UartIpcData> *linux_mcu_ipc::UartIpcConnection::s_connection(nullptr);