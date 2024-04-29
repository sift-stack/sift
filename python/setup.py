from setuptools import setup, find_packages

setup(
    name='sift',
    version='0.1',
    author='Sift Stack',
    description='Python Sift generated client from protobufs',
    install_requires = [
        "grpcio",
        "protobuf",
    ],
    packages=find_packages('lib'),
    package_dir={'': 'lib'},
)
