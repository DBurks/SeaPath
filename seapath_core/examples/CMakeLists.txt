# Define the example executable
add_executable(seapath_basic_example basic_usage.cpp)

# Link against our shared library target
target_link_libraries(seapath_basic_example PRIVATE seapath_lib)

# Crucial Step: Tell the example where to find the include directory
target_include_directories(seapath_basic_example PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)