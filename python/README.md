# Sift Python
[![pypi](https://img.shields.io/pypi/v/sift-stack-py)](https://pypi.org/project/sift-stack-py/)
[![pypi](https://img.shields.io/pypi/pyversions/sift-stack-py)](https://pypi.org/project/sift-stack-py/)

This library offers a Python API on top of Sift's protocol buffers to ergonomically interface with the Sift gRPC API.

## Installation

To install the Sift Python library:

```
$ pip install sift-stack-py
```

## Documentation

To build the documentation locally:

Clone the main `sift` repository:

```
$ git clone https://github.com/sift-stack/sift.git
```

Set your working directory to this project sub-directory:

```
$ cd python
```

Install [pdoc](https://pypi.org/project/pdoc/):

```
$ pip install pdoc
```

Run `pdoc`.

```
$ pdoc lib/sift_py
```

You may optionally set the host and port used by the `pdoc` web-server using the `-p` and `-h` options.
See `pdoc --help` for more info.
