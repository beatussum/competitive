cmake_minimum_required(VERSION 3.16)
cmake_policy(SET CMP0048 NEW)

project(@PROBLEM@
        HOMEPAGE_URL "@URL@"
        LANGUAGES CXX)

unset(CMAKE_CXX_FLAGS_RELEASE CACHE)
macro(add_gcc_cxx_flags _flags)
    if(CMAKE_COMPILER_IS_GNUCXX)
         set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${_flags}")
    endif()
endmacro()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED TRUE)
set(CMAKE_CXX_EXTENSIONS FALSE)

add_gcc_cxx_flags(-fdiagnostics-color=always)
add_gcc_cxx_flags(-fno-gnu-keywords -fno-operator-names)
add_gcc_cxx_flags(-funsigned-char -pipe -fmessage-length=75)
add_gcc_cxx_flags(-Wnoexcept -Wold-style-cast -Woverloaded-virtual)
add_gcc_cxx_flags(-Wall -Wpedantic)
add_gcc_cxx_flags(-Wdouble-promotion -Wimplicit-fallthrough=5 -Wignored-qualifiers)
add_gcc_cxx_flags(-Wmissing-include-dirs -Wuninitialized -Wsuggest-override)
add_gcc_cxx_flags(-Wsuggest-attribute=pure -Wsuggest-attribute=const)
add_gcc_cxx_flags(-Wsuggest-attribute=noreturn -Wsuggest-final-types)
add_gcc_cxx_flags(-Wsuggest-final-types -Wsuggest-final-methods)
add_gcc_cxx_flags(-Wsuggest-override -Wduplicated-branches)
add_gcc_cxx_flags(-Wduplicated-cond -Wno-div-by-zero -Wfloat-equal)
add_gcc_cxx_flags(-Wundef -Wconversion -Wmissing-field-initializers)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_gcc_cxx_flags(-O0)
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_gcc_cxx_flags(-O2)
    add_gcc_cxx_flags(-Werror -Wfatal-errors)
endif()

set(PROBLEM_CPP problem.cpp)
    
add_executable(problem "${PROBLEM_CPP}")
