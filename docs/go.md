# Sift Protobuf Installation for Go

Before proceeding with installation, you will need to ensure that you have the [buf CLI](https://buf.build/docs/installation) installed.

If `$ which buf` generates a path to the executable, you may proceed to the installation steps.

To install Sift protobufs in your project:

1. Clone this repository onto your local machine and `cd` into it:

```bash
$ git clone https://github.com/sift-stack/sift
$ cd sift
```

2. Assuming the path to the root of your Go project is `$PROJECT_DIR`, run the following command in the `sift` directory that you just cloned:

```bash
$ buf export protos --output=$PROJECT_DIR/protos --config protos/buf.yaml
```

The Sift protos and its imports can now be found in your `$PROJECT_DIR/protos` directory.

3. Copy the `buf` template for Go to your project directory:

```bash
$ cp buf_templates/buf.gen.go.yaml $PROJECT_DIR/buf.gen.yaml
```

4. `cd` into your Go project at `$PROJECT_DIR`.

5. Once inside of your Go project, you'll need to modify the `managed.enabled.go_package_prefix.default` value of your `buf.gen.yaml` file to
have the package prefix named after your Go module. If our Go module's name, for example, is `github.com/example_project`, then change the `buf.gen.yaml` to `github.com/example_project/gen/protos/go`. Your
`buf.gen.yaml` should now look like the following:

```yaml
version: v1
managed:
  enabled: true
  go_package_prefix:
    default: "github.com/example_project/gen/protos/go"
plugins:
  - plugin: buf.build/protocolbuffers/go:v1.28.1
    out: gen/protos/go
    opt: paths=source_relative
  - plugin: go-vtproto
    out: gen/protos/go
    opt: paths=source_relative
  - plugin: buf.build/grpc-ecosystem/gateway:v2.16.2
    out: gen/protos/go
    opt: paths=source_relative
```

Refer to your `go.mod` file for the name of your Go module.

6. Inside of the root of your project directory you may now compile your protobufs:

```bash
$ buf generate protos
```

Your project up to this point should look like the following (full depth not shown):

```
 example_project
 ├─ buf.gen.yaml
 ├─ gen
 │  └─ protos
 │     └─ go
 ├─ go.sum
 ├─ go.mod
 └─ protos
    ├─ protoc-gen-openapiv2
    │  └─ options
    ├─ google
    │  └─ api
    └─ sift
       ├─ runs
       ├─ notifications
       ├─ annotations
       ├─ users
       ├─ common
       ├─ assets
       ├─ tags
       └─ annotation_logs
```

7. Install any dependencies you might be missing that the generated code requires:

```bash
$ go get -d ./...
```

8. Now your project should be ready to use the generated Go code to interact with Sift's gRPC API. Please refer to the [example code](/go/examples) for usage.
