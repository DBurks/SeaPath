set(SEAPATH_EXAMPLE_SRCS
    basic_usage.cpp)

add_executable(seapath_basic_example ${SEAPATH_EXAMPLE_SRCS})

target_link_libraries(
    seapath_basic_example PRIVATE
    seapath
)