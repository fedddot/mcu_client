FetchContent_Declare(
    mcu_task_engine
    GIT_REPOSITORY  https://github.com/fedddot/mcu_task_engine.git
    GIT_TAG         main
)
FetchContent_GetProperties(mcu_task_engine)
if(NOT mcu_task_engine_POPULATED)
	FetchContent_Populate(mcu_task_engine)
	add_subdirectory(${mcu_task_engine_SOURCE_DIR} ${mcu_task_engine_BINARY_DIR} EXCLUDE_FROM_ALL)
endif()