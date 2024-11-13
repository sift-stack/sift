# Sift C++

This tutorial shows how to build a C++ application that uses the Sift gRPC API.

## Installation

##### Build gRPC and Protocol Buffers

Follow the [official gRPC C++ tutorial](https://grpc.io/docs/languages/cpp/quickstart/#install-grpc) to build and locally install gRPC and Protocol Buffers. You should be able to build and run the [helloworld](https://grpc.io/docs/languages/cpp/quickstart/#build-the-example) example before continuing with the next section.

Ensure that `$ which grpc_cpp_plugin` and `$ which protoc` generate a path to executables.

You will also need to ensure that you have the [buf CLI](https://buf.build/docs/installation) installed.

Verify that `$ which buf` generates a path to the executable, before proceeding to the compilation steps.

##### C++ Protobuf Compilation

1. Clone this repository onto your local machine and `cd` into it:

```bash
$ git clone https://github.com/sift-stack/sift
$ cd sift
```

2. Assuming the path to the directory where we'll build the package is `$PACKAGE_DIR`, run the following command in the `sift` directory that you just cloned:

```bash
$ buf export protos --output=$PACKAGE_DIR/protos --config protos/buf.yaml
```

The Sift protos and its imports can now be found in your `$PACKAGE_DIR/protos` directory.

3. Copy the `buf` template for C++ to `$PACKAGE_DIR` and update the `protoc_path` variable to point to the `protoc` executable.

```bash
$ cp buf_templates/buf.gen.cpp.yaml $PACKAGE_DIR/buf.gen.yaml
```

4. `cd` into `$PACKAGE_DIR`.

5. Once inside of `$PACKAGE_DIR`, ensure that `buf.gen.yaml` is at the root.

6. Compile your protobufs.

```bash
$ buf generate protos
```

The generated code will be in the `gen` directory. Copy `gen` into to any C++ projects that you want to build against.


## Examples

Various examples can be found in the [examples](./examples) directory. To run any of those examples clone this repo, follow the steps above, then do the following:

1. Copy the generated code into the example directory that you want to build:
```
$ cp -r $PACKAGE_DIR/gen $EXAMPLE
```

2. `cd` into the example
```
$ cd $EXAMPLE
```

3. Ensure the environment variable `MY_INSTALL_DIR` holds the path to [locally installed packages](https://grpc.io/docs/languages/cpp/quickstart/#setup).

4. Build the example using `cmake`:
```
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=$MY_INSTALL_DIR ..
cmake --build . -j 4
```

5. Run the example with required environment variables:
```
$ SIFT_URL=<grpc_api_url>:443 SIFT_API_KEY=<api_key> SIFT_ORGANIZATION_ID=<organization_id> ./Example
```