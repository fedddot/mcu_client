FetchContent_Declare(
    jsoncpp
    GIT_REPOSITORY  https://github.com/open-source-parsers/jsoncpp.git
    GIT_TAG         1.9.5
)
FetchContent_GetProperties(jsoncpp)
if(NOT jsoncpp_POPULATED)
	FetchContent_Populate(jsoncpp)
    set(JSONCPP_WITH_TESTS OFF)
    set(JSONCPP_WITH_POST_BUILD_UNITTEST OFF)
    set(JSONCPP_WITH_WARNING_AS_ERROR OFF)
    set(JSONCPP_WITH_STRICT_ISO OFF)
    set(JSONCPP_WITH_PKGCONFIG_SUPPORT OFF)
    set(JSONCPP_WITH_CMAKE_PACKAGE OFF)
    set(JSONCPP_WITH_EXAMPLE OFF)
    set(BUILD_SHARED_LIBS OFF)
    set(BUILD_STATIC_LIBS ON)
    set(BUILD_OBJECT_LIBS OFF)
	add_subdirectory(${jsoncpp_SOURCE_DIR} ${jsoncpp_BINARY_DIR} EXCLUDE_FROM_ALL)
endif()