cmake_minimum_required(VERSION 3.14)
project(IngestionExample)

set(CMAKE_CXX_STANDARD 17)

find_package(Protobuf CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)

# Include the generated files directory
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/gen/proto/cpp)

# Gather all generated .pb.cc and .grpc.pb.cc files
file(GLOB_RECURSE PROTO_SRC "${CMAKE_CURRENT_SOURCE_DIR}/gen/proto/cpp/*.pb.cc"
                             "${CMAKE_CURRENT_SOURCE_DIR}/gen/proto/cpp/*.grpc.pb.cc")
file(GLOB_RECURSE PROTO_HDR "${CMAKE_CURRENT_SOURCE_DIR}/gen/proto/cpp/*.pb.h"
                             "${CMAKE_CURRENT_SOURCE_DIR}/gen/proto/cpp/*.grpc.pb.h")

# Add your application source files here
set(SOURCE_FILES ingest.cc ${PROTO_SRC} ${PROTO_HDR})

add_executable(IngestionExample ${SOURCE_FILES})

target_link_libraries(IngestionExample PRIVATE gRPC::grpc++ protobuf::libprotobuf)
