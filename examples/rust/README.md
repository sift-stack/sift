# Sift Rust Example

To run this example ensure that you have Rust and the [buf CLI](https://buf.build/docs/installation) installed.

Once those are installed and your working directory is this project's root, compile the protobufs:

```bash
$ buf generate protos
```

Now execute the program by providing the partial string of the annotations you wish to query. In the following example
we'll be querying for all annotations whose name matches the `voltage` substring in a case-insensitive manner.

```bash
$ cargo run -- voltage
```
