cmake_minimum_required(VERSION 3.16)
cmake_policy(SET CMP0048 NEW)

project(2A LANGUAGES C)

unset(CMAKE_C_FLAGS_RELEASE CACHE)

macro(add_gcc_c_flags __flags)
    if(CMAKE_C_COMPILER_ID EQUAL "GNU")
         set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${__flags}")
    endif()
endmacro()

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED TRUE)
set(CMAKE_C_EXTENSIONS FALSE)

add_gcc_c_flags(-fdiagnostics-color=always)
add_gcc_c_flags(-fno-gnu-keywords -fno-operator-names)
add_gcc_c_flags(-funsigned-char -pipe -fmessage-length=75)
add_gcc_c_flags(-Wnoexcept -Wold-style-cast -Woverloaded-virtual)
add_gcc_c_flags(-Wall -Wpedantic)
add_gcc_c_flags(-Wdouble-promotion -Wimplicit-fallthrough=5 -Wignored-qualifiers)
add_gcc_c_flags(-Wmissing-include-dirs -Wuninitialized -Wsuggest-override)
add_gcc_c_flags(-Wsuggest-attribute=pure -Wsuggest-attribute=const)
add_gcc_c_flags(-Wsuggest-attribute=noreturn -Wsuggest-final-types)
add_gcc_c_flags(-Wsuggest-final-types -Wsuggest-final-methods)
add_gcc_c_flags(-Wsuggest-override -Wduplicated-branches)
add_gcc_c_flags(-Wduplicated-cond -Wno-div-by-zero -Wfloat-equal)
add_gcc_c_flags(-Wundef -Wconversion -Wmissing-field-initializers)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_gcc_c_flags(-O0)
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_gcc_c_flags(-O2)
    add_gcc_c_flags(-Werror -Wfatal-errors)
endif()

set(PROBLEM_C problem.c)
set(PROBLEM_H problem.h)

add_executable(problem "${PROBLEM_C}" "${PROBLEM_H}")
