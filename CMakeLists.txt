cmake_minimum_required(VERSION 3.12)

project(mcu_client C CXX ASM)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(client INTERFACE)
target_include_directories(client INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)

add_library(mcu_client INTERFACE)
target_include_directories(mcu_client INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(mcu_client INTERFACE client ipc_connection)

add_executable(mcu_client_app ${CMAKE_CURRENT_LIST_DIR}/src/mcu_client_app.cpp)
target_include_directories(mcu_client_app PRIVATE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(
    mcu_client_app
    PRIVATE
    mcu_client
    linux_ipc_connection
)

add_subdirectory(external)
add_subdirectory(ipc)