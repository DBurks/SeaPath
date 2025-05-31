include (FetchContent)

FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG        v1.17.0
)

FetchContent_MakeAvailable(googletest)

add_library(GTest::GTest INTERFACE IMPORTED)

set(SEAPATH_TEST_SRCS
    test_units.cpp
    test_geodesy.cpp
    test_calculations.cpp)

add_executable(seapath_tests ${SEAPATH_TEST_SRCS})


target_link_libraries(seapath_tests PRIVATE
        seapath
        GTest::gtest_main)

enable_testing()

include(GoogleTest)
gtest_discover_tests(seapath_tests)

