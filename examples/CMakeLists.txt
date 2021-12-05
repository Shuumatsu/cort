cmake_minimum_required(VERSION 3.5) 

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED True)

set(CMAKE_BUILD_TYPE Debug)

project (cort_examples) 


add_executable(basic src/basic.c) 
target_include_directories(basic
    PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}/lib
)
target_link_directories(basic
    PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}/lib
)
target_link_libraries(basic cort)

# add_executable(single_thread src/single_thread.c) 
# target_include_directories(single_thread
#     PRIVATE 
#         ${CMAKE_CURRENT_SOURCE_DIR}/lib
# )
# target_link_directories(single_thread
#     PRIVATE 
#         ${CMAKE_CURRENT_SOURCE_DIR}/lib
# )
# target_link_libraries(single_thread cort)