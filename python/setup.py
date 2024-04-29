from setuptools import setup, find_packages

setup(
    name='sift',
    version='0.1',
    author='Sift Stack',
    description='Python Sift generated client from protobufs',
    packages=find_packages('sift'),
    package_dir={'': 'sift'},
)
