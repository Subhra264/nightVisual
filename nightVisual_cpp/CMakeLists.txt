# Atleast CMake version 3.15 is required
cmake_minimum_required(VERSION 3.15)

project(NightVisual)

message(STATUS
    "The value of CMAKE_BINARY_DIR is ${CMAKE_BINARY_DIR}\n"
    "The build files supposed to be inside ${CMAKE_BINARY_DIR}/lib"
)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(LIBRARY_OUTPUT_PATH ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# TODO: Find some workaround to put the executable files
# inside example folder. For now, both the library and executables
# are located inside lib/Debug folder, so that the executable is
# able to find the PATH of the shared library.
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/lib)

# Check if CMAKE_INSTALL_PREFIX has default values
# If so, customize it
if(DEFINED CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
    
    # Customize the install directory
    set(CMAKE_INSTALL_PREFIX
        "${CMAKE_SOURCE_DIR}/install"
        CACHE PATH "Where the library will be installed to" FORCE
    )
else()
    message(
        STATUS
        "CMAKE_INSTALL_PREFIX was already set\n"
        "Current value: ${CMAKE_INSTALL_PREFIX}"
    )
endif()

# If the following option is not set ON, then no symbols
# will be exported by default on Windows. In that case,
# we would have to use __declspec(export) manually to every
# class to enable export on Windows. On Unix/Linux, all the
# symbols are exported by default.
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

add_library(${PROJECT_NAME} SHARED
    # src/nightvisual.cc
    # include/nightvisual.hh
)

target_sources(${PROJECT_NAME}
    PRIVATE
        src/nightVisual.cc
)

# without it public headers won't get installed
set(public_headers
    include/nightVisual.hh
)
# note that ${public_headers} has to be in quotes
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER "${public_headers}")

target_include_directories(${PROJECT_NAME}
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

add_subdirectory(example)