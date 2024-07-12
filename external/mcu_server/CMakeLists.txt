FetchContent_Declare(
    mcu_server
    GIT_REPOSITORY  https://github.com/fedddot/mcu_server.git
    GIT_TAG         main
)
FetchContent_GetProperties(mcu_server)
if(NOT mcu_server_POPULATED)
	FetchContent_Populate(mcu_server)
	add_subdirectory(${mcu_server_SOURCE_DIR} ${mcu_server_BINARY_DIR} EXCLUDE_FROM_ALL)
endif()