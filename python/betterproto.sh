# Create a temporary directory for all protos
mkdir -p ./tmp/proto-include
# Copy or clone required dependencies
git clone https://github.com/googleapis/googleapis /tmp/proto-include/googleapis
git clone https://github.com/grpc-ecosystem/grpc-gateway /tmp/proto-include/grpc-gateway

mkdir -p ./python/betterproto_lib
# Run protoc with all include paths
protoc \
-I ./protos \
-I /tmp/proto-include/googleapis \
-I /tmp/proto-include/grpc-gateway \
--python_betterproto_out=./python/betterproto_lib \
./**/*.proto


#--python_betterproto_opt=pydantic_dataclasses \
