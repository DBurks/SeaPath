cmake_minimum_required(VERSION 3.15...3.28)

project(SeaPath 
        VERSION 0.1.0
        DESCRIPTION "A C++ Library for Marine Navigation Calculations"
        LANGUAGES CXX)

# BUMPED TO C++20 for Concepts and Advanced Scheduling Constraints
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Optimize release builds and make sure debug info for debug builds
if (CMAKE_BUILD_TYPE STREQUAL "Release")
    message(STATUS "Building in Release Mode. Optimizations enabled.")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3")
elseif(CMAKE_BUILD_TYPE STREQUAL "Debug")
    message(STATUS "Building in Debug Mode. Debug info is enabled.")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -g -O0")
endif()

# Threading support needed for real-time executive loops
find_package(Threads REQUIRED)

# Location of public headers
set(SEAPATH_INCLUDE_INSTALL_DIR "${CMAKE_INSTALL_INCLUDEDIR}/seapath")

# Subdirectories for building components
add_subdirectory(src)
add_subdirectory(tests)
add_subdirectory(examples)