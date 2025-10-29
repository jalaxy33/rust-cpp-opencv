# =============================================================================
# Preamble
# =============================================================================

cmake_minimum_required(VERSION 3.10.0)

# vcpkg integration
set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")
set(VCPKG_INSTALLED_DIR "$ENV{VCPKG_ROOT}/installed")   # use system-wide vcpkg installation

# Force vcpkg to use Release libraries even in Debug mode
# This is necessary to match Rust's CRT linkage (always Release)
set(CMAKE_MAP_IMPORTED_CONFIG_DEBUG Release)
set(CMAKE_MAP_IMPORTED_CONFIG_RELWITHDEBINFO Release)

# Disable vcpkg's automatic DLL copy mechanism
# We will manually manage DLL copying for all configurations
set(VCPKG_APPLOCAL_DEPS OFF CACHE BOOL "Disable vcpkg automatic DLL copy" FORCE)

# generate `compile_commands.json` for clangd
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# =============================================================================
# Project Configuration
# =============================================================================

project(opencv-demo VERSION 0.1.0 LANGUAGES C CXX)
set(CMAKE_CXX_STANDARD 17)

# =============================================================================
# Find Packages
# =============================================================================

find_package(OpenCV REQUIRED)
link_libraries(${OpenCV_LIBS})


find_package(ITK CONFIG REQUIRED)
include(${ITK_USE_FILE})
link_libraries(${ITK_LIBRARIES})


# =============================================================================
# Rust Configuration
# =============================================================================

function(setup_rust_library targetname)
    # Fix Windows MSVC runtime library compatibility
    # Always use /MD (Release CRT) to match Rust and vcpkg Release libraries
    if(WIN32 AND MSVC)
        target_compile_options(${targetname} PRIVATE /MD)
        # Disable iterator debug level in Debug builds to match Release libraries
        # Use generator expression to work with multi-config generators like Visual Studio
        target_compile_definitions(${targetname} PRIVATE 
            $<$<CONFIG:Debug>:_ITERATOR_DEBUG_LEVEL=0>
        )
    endif()

    # Ensure the Rust library is built
    set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
    set(rust_lib_dir ${RUST_TARGET_DIR}/release)   # always use release build for compatibility

    if(NOT EXISTS ${rust_lib_dir})
        message(WARNING "Rust library not found. Building Rust library now...")
        execute_process(
            COMMAND cargo build --release
            WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        )
    endif()

    # Search for the Rust library
    file(GLOB rust_lib_files
        "${rust_lib_dir}/${CMAKE_STATIC_LIBRARY_PREFIX}*${CMAKE_SHARED_LIBRARY_SUFFIX}"
    )

    # link Rust library
    foreach(rust_lib IN LISTS rust_lib_files)
        if (WIN32)
            set(rust_lib "${rust_lib}.lib")
        endif()
        target_link_libraries(${targetname} PRIVATE ${rust_lib})
    endforeach()


    # Set include directories for cxx generated headers
    file(GLOB cxxbridge_include_dirs
        "${RUST_TARGET_DIR}/cxxbridge/*/src"
    )
    target_include_directories(${targetname} PRIVATE
        ${RUST_TARGET_DIR}/cxxbridge
        ${RUST_TARGET_DIR}/cxxbridge/rust
        ${cxxbridge_include_dirs}
    )

    # Gather cxxbridge generated source files
    file(GLOB cxxbridge_src
        "${RUST_TARGET_DIR}/cxxbridge/*/src/*.rs.cpp"
        "${RUST_TARGET_DIR}/cxxbridge/*/src/*.rs.cc"
    )

    if (cxxbridge_src)
        target_sources(${targetname} PRIVATE ${cxxbridge_src})
    endif()

    # link cxx library
    file(GLOB cxx_lib_files
        "${rust_lib_dir}/build/cxx-*/out/*${CMAKE_STATIC_LIBRARY_SUFFIX}"
    )
    foreach(cxx_lib IN LISTS cxx_lib_files)
        target_link_libraries(${targetname} PRIVATE ${cxx_lib})
    endforeach()

    # Copy DLL on Windows
    if(WIN32)
        add_custom_command(TARGET ${targetname} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${rust_lib_files}"
            $<TARGET_FILE_DIR:${targetname}>
            COMMENT "Copying Rust DLL for $<CONFIG>."
        )
    endif()

endfunction()


# =============================================================================
# Export vcpkg packages DLL (for all configurations)
# =============================================================================

# Function to copy vcpkg Release DLLs for all build configurations
# Since VCPKG_APPLOCAL_DEPS is disabled, we manually manage DLL copying
# All configurations use Release DLLs due to CMAKE_MAP_IMPORTED_CONFIG_DEBUG=Release
function(export_vcpkg_package_dll targetname)
    if(NOT WIN32)
        return()
    endif()

    # Fix Windows MSVC runtime library compatibility
    # Always use /MD (Release CRT) to match Rust and vcpkg Release libraries
    if(WIN32 AND MSVC)
        target_compile_options(${targetname} PRIVATE /MD)
        # Disable iterator debug level in Debug builds to match Release libraries
        # Use generator expression to work with multi-config generators like Visual Studio
        target_compile_definitions(${targetname} PRIVATE 
            $<$<CONFIG:Debug>:_ITERATOR_DEBUG_LEVEL=0>
        )
    endif()
    
    # Find PowerShell
    find_program(POWERSHELL_PATH NAMES pwsh powershell)
    
    if(NOT POWERSHELL_PATH)
        message(WARNING "PowerShell not found, will copy all Release DLLs for all configurations")
        # Fallback: copy all Release DLLs for all configurations
        add_custom_command(TARGET ${targetname} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_directory
                "${VCPKG_INSTALLED_DIR}/x64-windows/bin"
                "$<TARGET_FILE_DIR:${targetname}>"
            COMMENT "Copying all vcpkg Release DLLs for $<CONFIG> build"
        )
    else()
        # Use vcpkg's applocal.ps1 to copy only necessary Release DLLs
        # Always use /bin (Release) directory since we always link against Release libraries
        set(VCPKG_APPLOCAL_SCRIPT "$ENV{VCPKG_ROOT}/scripts/buildsystems/msbuild/applocal.ps1")
        add_custom_command(TARGET ${targetname} POST_BUILD
            COMMAND "${POWERSHELL_PATH}" -noprofile -executionpolicy Bypass
                -file "${VCPKG_APPLOCAL_SCRIPT}"
                -targetBinary "$<TARGET_FILE:${targetname}>"
                -installedDir "${VCPKG_INSTALLED_DIR}/x64-windows/bin"
                -tlogFile "$<TARGET_FILE_DIR:${targetname}>/${targetname}.tlog"
            COMMENT "Copying necessary vcpkg Release DLLs for $<CONFIG> build."
            VERBATIM
        )
    endif()
endfunction()


# =============================================================================
# Target Configuration
# =============================================================================

set(targetname demo)
add_executable(${targetname} src/main.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)

setup_rust_library(${targetname})

# Set global definitions for c++ codes
target_compile_definitions(${targetname} PRIVATE PROJECT_ROOT="${CMAKE_SOURCE_DIR}")

# Manually copy vcpkg Release DLLs for all configurations
# Required since VCPKG_APPLOCAL_DEPS is disabled to avoid conflicts
# All configs use Release libraries due to CMAKE_MAP_IMPORTED_CONFIG_DEBUG=Release
export_vcpkg_package_dll(${targetname})


# =============================================================================
# Test Configuration
# =============================================================================

enable_testing()

# test function (no external dependencies)
function(add_test_target test_name test_file)
    add_executable(${test_name} ${test_file})
    target_include_directories(${test_name} PRIVATE ${CMAKE_SOURCE_DIR}/include)
    target_compile_definitions(${test_name} PRIVATE PROJECT_ROOT="${CMAKE_SOURCE_DIR}")
    add_test(NAME ${test_name} COMMAND ${test_name})
endfunction()

add_test_target(test_common tests/test_common.cpp)

