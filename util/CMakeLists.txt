add_library(uart INTERFACE)
target_sources(uart INTERFACE ${CMAKE_CURRENT_LIST_DIR}/src/uart.cpp)
target_include_directories(uart INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(
    uart
    INTERFACE
    listener
    mcu_client_types
)

add_library(uart_connection INTERFACE)
target_include_directories(uart_connection INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(
    uart_connection
    INTERFACE
    custom_listener
    custom_receiver
    mcu_client_types
    server_connection
    uart
)

add_library(gpi_proxy INTERFACE)
target_include_directories(gpi_proxy INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(
    gpi_proxy
    INTERFACE
    gpi
    client
    data_lib
    parser
    serializer
    mcu_task_type
)

add_library(gpo_proxy INTERFACE)
target_include_directories(gpo_proxy INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(
    gpo_proxy
    INTERFACE
    gpo
    client
    data_lib
    parser
    serializer
    mcu_task_type
)

add_subdirectory(tests EXCLUDE_FROM_ALL)