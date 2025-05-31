# define the source files for this project

set(SEAPATH_LIB_SRCS
    units.cpp
    geodesy.cpp
    calculations.cpp)

# add SeaPAth Library target
add_library(seapath SHARED ${SEAPATH_LIB_SRCS})

target_include_directories(seapath PUBLIC 
        $<INSTALL_INTERFACE:${SEAPATH_INCLUDE_INSTALL_DIR}>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/../include>)

target_compile_definitions(seapath PRIVATE SEAPATH_EXPORTS)

install(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/../include/seapath/
        DESTINATION ${SEAPATH_INCLUDE_INSTALL_DIR})
install(TARGETS seapath EXPORT SeaPathTargets
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})  

install(EXPORT SeaPathTargets
        FILE SeaPathTargets.cmake
        NAMESPACE seapath::
        DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/seapath)

