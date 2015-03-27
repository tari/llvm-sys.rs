cmake_minimum_required(VERSION 2.6)

find_package(LLVM REQUIRED CONFIG)

include_directories(${LLVM_INCLUDE_DIRS})
add_definitions(${LLVM_DEFINITIONS})

add_library(targetwrappers STATIC
            target.c)
