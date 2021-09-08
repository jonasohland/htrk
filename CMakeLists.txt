cmake_minimum_required(VERSION 3.19)

project(htrk LANGUAGES C CXX VERSION 0.0.1)

set(htrk_default_build_type "Release")
if(EXISTS "${CMAKE_SOURCE_DIR}/.git")
  set(htrk_default_build_type "Debug")
endif()

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "Setting build type to '${htrk_default_build_type}' as none was specified.")
  set(CMAKE_BUILD_TYPE "${htrk_default_build_type}" CACHE
      STRING "Choose the type of build." FORCE)
  set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS
    "Debug" "Release" "MinSizeRel" "RelWithDebInfo")
endif()

message (STATUS "Checking for cargo")

# ----------- Check for cargo ---------------

find_program(htrk_cargo_path cargo)

if (${htrk_cargo_path} STREQUAL htrk_cargo_path-NOTFOUND)
    message (FATAL_ERROR "Could not find cargo. Please install rust")
endif()

execute_process (COMMAND cargo --version 
                 OUTPUT_VARIABLE htrk_cargo_version_output
                 RESULT_VARIABLE htrk_cargo_version_result
                 OUTPUT_STRIP_TRAILING_WHITESPACE)         

string (REPLACE "cargo" "" htrk_cargo_version_output ${htrk_cargo_version_output})
string (STRIP "${htrk_cargo_version_output}" htrk_cargo_version_output)

message (STATUS "Checking for cargo - done. Version: ${htrk_cargo_version_output}")

# --------- Check for cbindgen --------------

message (STATUS "Checking for cbindgen") 

find_program(htrk_cbindgen_path cbindgen)

if (${htrk_cbindgen_path} STREQUAL htrk_cbindgen_path-NOTFOUND)
    message (FATAL_ERROR "Could not find cbindgen. Try installing it with cargo")
endif()

execute_process (COMMAND cbindgen --version 
                 OUTPUT_VARIABLE htrk_cbindgen_version_output
                 RESULT_VARIABLE htrk_cbindgen_version_result
                 OUTPUT_STRIP_TRAILING_WHITESPACE)         

string (REPLACE "cbindgen" "" htrk_cbindgen_version_output "${htrk_cbindgen_version_output}")
string (STRIP "${htrk_cbindgen_version_output}" htrk_cbindgen_version_output)

message (STATUS "Checking for cbindgen - done. Version: ${htrk_cbindgen_version_output}") 

# --------- Check for cbindgen --------------

include(ExternalProject)

set (htrk_cargo_build_dir ${CMAKE_CURRENT_BINARY_DIR}/cargo/targets)

message(STATUS "Cargo build dir: ${htrk_cargo_build_dir}")


# --------- Cargo project setup -------------

cmake_policy(SET CMP0114 NEW)

ExternalProject_Add (htrkapi 
                     SOURCE_DIR ${CMAKE_CURRENT_LIST_DIR}
                     BUILD_COMMAND ${htrk_cargo_path} 
                        build --release
                        --target-dir "${htrk_cargo_build_dir}"
                        --package htrkapi
                     INSTALL_COMMAND ""
                     CONFIGURE_COMMAND "")

file (GLOB_RECURSE htrk_rs_files ${CMAKE_CURRENT_LIST_DIR}/htrk/src/*.rs)
file (GLOB_RECURSE htrkapi_rs_files ${CMAKE_CURRENT_LIST_DIR}/htrkapi/src/*.rs)

set (htrk_api_depends 
     ${htrk_rs_files} 
     ${htrkapi_rs_files} 
     "${CMAKE_CURRENT_LIST_DIR}/Cargo.toml" 
     "${CMAKE_CURRENT_LIST_DIR}/htrk/Cargo.toml" 
     "${CMAKE_CURRENT_LIST_DIR}/htrkapi/Cargo.toml")

ExternalProject_Add_Step(htrkapi generate_headers
                         DEPENDERS build
                         DEPENDS ${htrk_api_depends}
                         WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
                         COMMAND ${htrk_cbindgen_path}
                             --crate htrkapi 
                             --cpp-compat
                             --style both 
                             --lang c
                             --output "${CMAKE_CURRENT_BINARY_DIR}/cargo/include/htrkapi/htrkapi.h")

ExternalProject_Add_StepDependencies(htrkapi build ${htrk_api_depends})

add_library(htrk SHARED IMPORTED)

add_dependencies(htrk htrkapi)

set_target_properties(htrk PROPERTIES 
                        IMPORTED_LOCATION ${htrk_cargo_build_dir}/release/libhtrkapi.a)

file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/cargo/include")
target_include_directories(htrk INTERFACE "${CMAKE_CURRENT_BINARY_DIR}/cargo/include")

add_subdirectory(${CMAKE_CURRENT_LIST_DIR}/test-c)