add_library(uart INTERFACE)
target_sources(uart INTERFACE ${CMAKE_CURRENT_LIST_DIR}/src/uart.cpp)
target_include_directories(uart INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(uart INTERFACE listener mcu_client_types)

add_library(uart_connection INTERFACE)
target_include_directories(uart_connection INTERFACE ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(uart_connection INTERFACE uart server_connection)

add_subdirectory(tests EXCLUDE_FROM_ALL)