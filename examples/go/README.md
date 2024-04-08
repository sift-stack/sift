# Sift Go Example

To run this example ensure that you have Go and the [buf CLI](https://buf.build/docs/installation) installed as well as having your
Sift API key ready. If you need a Sift API key please refer to [these instructions](https://help.siftstack.com/en/articles/8600475-api-keys).

Once those are installed and your working directory is this project's root, compile the protobufs:

```bash
$ buf generate protos
```

Install dependencies:

```bash
$ go get -d ./...
```

Create your `.env` file:

```bash
$ cp .env-example .env
```

Be sure to set the appropriate environment variables in your `.env` file depending on the environment you're using. Comments
meant to serve as guides can be found in the `.env-example` file.

Now execute the program by providing the partial string of the annotations you wish to query. In the following example
we'll be querying for all annotations whose name matches the `voltage` substring in a case-insensitive manner.

```bash
$ go run . voltage
```
