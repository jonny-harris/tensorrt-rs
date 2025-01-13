cmake_minimum_required(VERSION 3.10)
project(LibTRT LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD 17)

# Set CUDA paths (use the appropriate version)
set(CUDA_TOOLKIT_PATH "/usr/local/cuda-12.2")
set(CUDART_LIBRARY_PATH "/usr/local/cuda-12.2/lib64")

# Specify CUDA include directories
include_directories(
    ${CUDA_TOOLKIT_PATH}/include
    ${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES}
)

# Specify the location of the CUDA runtime library
find_library(CUDART_LIBRARY cudart ${CUDART_LIBRARY_PATH})

# Define source files
file(GLOB source_files
        "TRTLogger/*.h"
        "TRTLogger/*.cpp"
        "TRTRuntime/*.h"
        "TRTRuntime/*.cpp"
        "TRTCudaEngine/*.h"
        "TRTCudaEngine/*.cpp"
        "TRTContext/*.h"
        "TRTContext/*.cpp"
        "TRTUffParser/*.h"
        "TRTUffParser/*.cpp"
        "TRTDims/*.h"
        "TRTBuilder/*.h"
        "TRTBuilder/*.cpp"
        "TRTNetworkDefinition/*.h"
        "TRTNetworkDefinition/*.cpp"
        "*.h"
)

# Add the library target
add_library(trt-sys STATIC ${source_files})

# Link against necessary libraries
target_link_libraries(trt-sys PRIVATE nvinfer ${CUDART_LIBRARY})

# Ensure CUDA-specific paths are set correctly
include_directories(${CMAKE_CUDA_TOOLKIT_INCLUDE_DIRECTORIES})

# Install the target
install(TARGETS trt-sys DESTINATION .)

# Optionally, you can also specify any other needed environment paths here if required for your build process.
