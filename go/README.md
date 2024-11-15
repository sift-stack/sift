# Sift Go

This library offers a Go API on top of Sift's protocol buffers to ergonomically interface with the Sift gRPC API.

## Installation

```
$ go get github.com/sift-stack/sift/go
```

## Examples

Various examples can be found in the [examples](./examples) directory. To run any of those examples clone this repo do the following:

```
$ SIFT_URI=<sift uri> SIFT_API_KEY=<api key> go run examples/ping/main.go 
```
