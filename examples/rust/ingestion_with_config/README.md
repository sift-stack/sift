# Sift Rust Ingestion Example

In this example we're going to simulate data ingestion for the asset and channels found in the [lunar_rover0.yaml](./configs/lunar_rover0.yml) file.

To run this example proceed with the following steps and be sure to have your
Sift API key ready. If you need a Sift API key please refer to [these instructions](https://help.siftstack.com/en/articles/8600475-api-keys).

Create your `.env` file:

```bash
$ cp .env-example .env
```

Be sure to set the appropriate environment variables in your `.env` file depending on the environment you're using. Comments
meant to serve as guides can be found in the `.env-example` file.

Run the example

```bash
$ cargo run
```

You should now see the `LunarRover0` asset and its associated run appear in the Sift application.
