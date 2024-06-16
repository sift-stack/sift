from setuptools import setup, find_packages

setup(
    name="sift_protos",
    version="0.1",
    author="Sift Stack",
    author_email="siftstack@example.com",
    description="Sift generated protos",
    packages=find_packages("gen"),
    package_dir={"": "gen"},
)
