# Sift Protobuf Installation for Rust

Before proceeding with installation, you will need to ensure that you have the [buf CLI](https://buf.build/docs/installation) installed.

If `$ which buf` generates a path to the executable, you may proceed to the installation steps.

To install Sift protobufs in your project:

1. Clone this repository onto your local machine and `cd` into it:

```bash
$ git clone https://github.com/sift-stack/sift
$ cd sift
```

2. Assuming the path to the root of your Rust project is `$PROJECT_DIR`, run the following command in the `sift` directory that you just cloned:

```bash
$ buf export protos --output=$PROJECT_DIR/protos --config protos/buf.yaml
```

The Sift protos can and its imports can now be found in your `$PROJECT_DIR/protos` directory.

3. Copy the `buf` template for Rust to your project directory:

```bash
$ cp buf_templates/buf.gen.go.yaml $PROJECT_DIR/buf.gen.yaml
```

4. `cd` into your Rust project at `$PROJECT_DIR`.

5. Once inside of your Rust project, declare a module called `gen` in your `main.rs` (unless you're crate is a lib-crate) and create a `src/gen/mod.rs` file.

```rust
// main.go

/// Sift generated code
mod gen;
```

Refer to the `buf.gen.yaml` in your project root if you need to modify the output path for the compiled protos.


6. Inside of the root of your project directory you may now compile your protobufs:

```bash
$ buf generate protos
```

Your project up to this point should look like the following (full depth not shown):

```
 example_project
 ├─ src
 │  ├─ main.rs
 │  └─ gen
 │     ├─ sift.common.type.v1.rs
 │     ├─ sift.runs.v2.rs
 │     ├─ sift.annotation_logs.v1.rs
 │     ├─ sift.runs.v2.tonic.rs
 │     ├─ sift.users.v2.rs
 │     ├─ mod.rs
 │     ├─ sift.tags.v1.rs
 │     ├─ sift.assets.v1.tonic.rs
 │     ├─ sift.assets.v1.rs
 │     ├─ sift.notifications.v1.tonic.rs
 │     ├─ sift.users.v2.tonic.rs
 │     ├─ sift.annotation_logs.v1.tonic.rs
 │     ├─ grpc.gateway.protoc_gen_openapiv2.options.rs
 │     ├─ sift.notifications.v1.rs
 │     ├─ sift.annotations.v1.tonic.rs
 │     ├─ google.api.rs
 │     └─ sift.annotations.v1.rs
 ├─ buf.gen.yaml
 ├─ README.md
 ├─ Cargo.lock
 └─ Cargo.toml

2 directories, 22 files
```

7. Ensure you have the following dependencies installed:

```toml
[package]
name = "sift_cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
async-trait = "0.1.79"
prost = "0.12.4"
prost-types = "0.12.4"
tonic = { version = "0.11.0", features = ["tls", "tls-roots", "tls-webpki-roots"] }
```

8. Declare the modules that will import the generated code in your `src/gen/mod.rs`. For example, we wish to use the generated `annotations` code for this example:

```rust
#[path = "sift.annotations.v1.rs"]
pub mod annotations;
```

9. Now your project should be ready to use the generated Rust code to interact with Sift's gRPC API. Please refer to the [example code](/examples/rust/) for usage.
