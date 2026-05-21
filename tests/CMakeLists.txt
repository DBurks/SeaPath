include(FetchContent)

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        v1.17.0
)

# This automatically handles internal GTest include paths and targets
FetchContent_MakeAvailable(googletest)

set(SEAPATH_TEST_SRCS
    test_units.cpp
    test_geodesy.cpp
    test_calculations.cpp)

# Define the test executable
add_executable(seapath_tests ${SEAPATH_TEST_SRCS})

# Pass your internal header directory path to the test files
target_include_directories(seapath_tests PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

# LINKER FIX: Changed 'seapath' to 'seapath_lib' to match your actual target in src/
target_link_libraries(seapath_tests PRIVATE
    seapath_lib
    GTest::gtest_main)

enable_testing()

include(GoogleTest)
gtest_discover_tests(seapath_tests)