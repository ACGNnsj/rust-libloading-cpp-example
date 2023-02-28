cmake_minimum_required(VERSION 3.25)
project(spsolver)

set(CMAKE_CXX_STANDARD 23)
find_package(Eigen3 CONFIG REQUIRED)
add_library(spsolver SHARED spsolver.cpp)
target_link_libraries(spsolver PRIVATE Eigen3::Eigen)
