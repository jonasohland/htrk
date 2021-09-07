add_executable(htrk_test_c ${CMAKE_CURRENT_LIST_DIR}/src/main.c)

target_link_libraries(htrk_test_c PRIVATE htrk)