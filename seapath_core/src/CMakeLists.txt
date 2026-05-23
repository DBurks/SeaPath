# =================================================================
# 1. Existing Shared Library Configuration (Preserved)
# =================================================================
add_library(seapath_lib SHARED 
    calculations.cpp 
    geodesy.cpp 
    units.cpp
)

# Set up include paths so consumers can find public headers
target_include_directories(seapath_lib PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>
    $<INSTALL_INTERFACE:include>
)

# Add an alias to match your build setup if needed by tests/examples
add_library(SeaPath::seapath_lib ALIAS seapath_lib)

# =================================================================
# 2. Real-Time Daemon Configuration (New)
# =================================================================
# Define our real-time simulation executive node executable
add_executable(seapath_rt_node 
    main.cpp
)

# Link both your math library and the OS system threading engine
target_link_libraries(seapath_rt_node PRIVATE 
    seapath_lib 
    Threads::Threads
)

# Ensure the executable can locate the executive header files cleanly
target_include_directories(seapath_rt_node PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

# =================================================================
# 3. Explicit Instrumentation Gate (Forced Globally)
# =================================================================
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_options(seapath_lib PRIVATE --coverage)
    target_link_options(seapath_lib PUBLIC --coverage)
    
    target_compile_options(seapath_rt_node PRIVATE --coverage)
    target_link_options(seapath_rt_node PRIVATE --coverage)
endif()