# Sift Protobuf Installation for Python

Before proceeding with installation, you will need to ensure that you have the [buf CLI](https://buf.build/docs/installation) installed.

If `$ which buf` generates a path to the executable, you may proceed to the installation steps.

Installing Sift protos into your project is done in two phases:
- The first phase is generating a local Python package out of the code generated from the Sift protos.
- The second phase is using `pip install` to add the Sift generated code as a dependency along with any other missing dependencies.

To get started, make sure that you have `python3` installed as well as an empty directory that will serve as the place to build the
Sift python package. Inside of that directory, it is recommended that you create a virtual environment. Example:

```bash
$ python -m venv venv
```

Once that is done, proceed with the following steps.

1. Clone this repository onto your local machine and `cd` into it:

```bash
$ git clone https://github.com/sift-stack/sift
$ cd sift
```

2. Assuming the path to the aforementioned empty directory where we'll build the package is `$PACKAGE_DIR`, run the following command in the `sift` directory that you just cloned:

```bash
$ buf export protos --output=$PACKAGE_DIR/protos --config protos/buf.yaml
```

The Sift protos can and its imports can now be found in your `$PACKAGE_DIR/protos` directory.

3. Copy the `buf` template for Python to `$PACKAGE_DIR`

```bash
$ cp buf_templates/buf.gen.python.yaml $PACKAGE_DIR/buf.gen.yaml
```

Copy `setup.py` as well:

```bash
$ cp scripts/setup.py $PACKAGE_DIR/setup.py
```

4. `cd` into `$PACKAGE_DIR`.

5. Once inside of `$PACKAGE_DIR`, ensure that `buf.gen.yaml` is at the root.

6. Compile your protobufs.

```bash
$ buf generate protos
```

Your project up to this point should look like the following (full depth not shown and virtual env files omitted):

```
 python_example
 ├─ buf.gen.yaml
 ├─ gen
 │  ├─ protoc_gen_openapiv2
 │  │  └─ options
 │  ├─ google
 │  │  └─ api
 │  └─ sift
 │     ├─ notifications
 │     ├─ data
 │     ├─ runs
 │     ├─ users
 │     ├─ rules
 │     ├─ assets
 │     ├─ tags
 │     ├─ calculated_channels
 │     ├─ annotations
 │     ├─ common
 │     └─ annotation_logs
 └─ protos
    ├─ protoc-gen-openapiv2
    │  └─ options
    ├─ google
    │  └─ api
    └─ sift
       ├─ common
       ├─ notifications
       ├─ tags
       ├─ runs
       ├─ assets
       ├─ data
       ├─ rules
       ├─ users
       ├─ calculated_channels
       ├─ annotations
       └─ annotation_logs
```

7. Execute the following script from your package root (i.e. `$PACKAGE_DIR`) to turn each directory in the generated code into a Python module:

```bash
for dir in $(find gen -type d); do
  touch $dir/__init__.py
done
```

8. Ensure that your virtual environment is active:

```bash
$ source venv/bin/activate
```

9. Install the following dependencies:

```bash
$  pip install build protobuf grpcio
```

10. Inspect `setup.py` and customize the metadata if necessary.

11. Build the source distribution and wheel to generate the Python package:

```bash
$ python -m build --sdist && python -m build --wheel
```

12. Once that is complete, you should now have a `dist` in `$PACKAGE_DIR` which contains your Python package. For a given `setup.py` that looks like this:

```python
from setuptools import setup, find_packages

setup(
    name='sift_protos',
    version='0.1',
    author='Sift Stack',
    author_email='siftstack@example.com',
    description='Sift generated protos',
    packages=find_packages('gen'),
    package_dir={'': 'gen'},
)
```

the generated wheel file should be outputted into `$PACKAGE_DIR/dist/sift_protos-0.1-py3-none-any.whl`.

13. Now from your actual Python project, you can install the newly generated package via `pip`:

```bash
$ pip install $PACKAGE_DIR/sift_protos-0.1-py3-none-any.whl
```

14. Now your project should be ready to use the generated Python code to interact with Sift's gRPC API. Please refer to the [example code](/examples/python) for usage.
