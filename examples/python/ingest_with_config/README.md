# Ingesting Data with a Config

This example demonstrates how to use Sift protos in Python to ingest data using an ingestion config. The script
contains doc-strings that you may find helpful.

To run this example proceed with the following steps and be sure to have your
Sift API key ready. If you need a Sift API key please refer to [these instructions](https://help.siftstack.com/en/articles/8600475-api-keys).

Activate your virtual environment:

```bash
$ python -m venv venv
$ source venv/bin/activate
```

Install dependencies:

```bash
$ pip install -r requirements.txt
```

Create your `.env` file:

```bash
$ cp .env-example .env
```

Be sure to set the appropriate environment variables in your `.env` file depending on the environment you're using. Comments
meant to serve as guides can be found in the `.env-example` file.

Before executing the script, there is a class in [main.py](main.py) called `ExampleTestRunConfig` that you are free to edit
to change the run names, asset names, etc..

To execute the script run the following:

```bash
$ python main.py
```

Once the script finishes executing, you should see a new run in the Sift UI containing newly telemetered data.
