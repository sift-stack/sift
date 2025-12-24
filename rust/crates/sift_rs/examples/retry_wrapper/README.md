# retry_wrapper

This example demonstrates how to use the retry wrapper functionality for gRPC wrapper services in `sift_rs`.

The retry mechanism automatically retries failed operations on transient gRPC errors (like `Unavailable`, `ResourceExhausted`, or `DeadlineExceeded`) with configurable exponential backoff.

## Usage

```bash
SIFT_API_KEY="$MY_API_KEY" SIFT_URI="$SIFT_GRPC_URL" cargo r --example retry_wrapper
```

## What This Example Shows

1. **Default Retry Configuration**: Using the default retry settings (3 attempts, 100ms base delay, exponential backoff)

2. **Custom Retry Configuration**: Creating a custom retry configuration with specific backoff settings for your use case

3. **Multiple Wrapper Types**: Examples using both `AssetServiceWrapper` and `IngestionConfigServiceWrapper` to show the retry mechanism works with any wrapper service

## Important Notes

- **Idempotency**: Only use retries for idempotent operations. Non-idempotent operations may be executed multiple times if retries occur.

- **Streaming RPCs**: This retry mechanism does not support streaming RPCs. Streaming calls require recreating the stream and may have side effects.

- **Retryable Errors**: The default retry decider will retry on:
  - `tonic::Code::Unavailable`
  - `tonic::Code::ResourceExhausted`
  - `tonic::Code::DeadlineExceeded`
  - Certain `ErrorKind` values like `GrpcConnectError`, `RetrieveAssetError`, etc.

