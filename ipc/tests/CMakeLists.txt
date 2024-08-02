add_executable(
	linux_ipc_tests
	${CMAKE_CURRENT_LIST_DIR}/src/main.cpp
	${CMAKE_CURRENT_LIST_DIR}/src/ut_linux_ipc_connection.cpp
)

target_link_libraries(
	linux_ipc_tests
	PRIVATE
	linux_ipc_connection
	gtest
)
