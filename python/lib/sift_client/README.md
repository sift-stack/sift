# Sift Client Library

This library provides a generic low-level wrapper pattern, REST service, and gRPC service for interacting with Sift APIs.

## Overview

The library is organized into three main components:

1. **Wrapper Pattern**: A generic low-level wrapper for API clients that handles common concerns like configuration, error handling, and retries.
2. **REST Service**: A client for making REST API calls to Sift services.
3. **gRPC Service**: A client for making gRPC API calls to Sift services.

## Installation

```bash
pip install sift-client
```

For OpenSSL support:

```bash
pip install sift-client[openssl]
```

## Usage

### Wrapper Pattern

The wrapper pattern provides a common interface for different types of API clients (REST, gRPC, etc.) and handles common concerns like configuration, error handling, and retries.

```python
from sift_client.transport.rest_transport import RestClient
from sift_client.transport.grpc_transport import GrpcClient
from sift_client.wrapper.utils import ClientError, retry

# See the REST and gRPC client implementations for examples of how to use the wrapper pattern.
```

### REST Service

The REST service provides a client for making REST API calls to Sift services.

```python
from sift_client.rest.client import RestClient, RestConfig

# Create a REST client configuration
config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
    timeout=30,
    use_ssl=True,
    max_retries=3,
    retry_backoff_factor=1.0,
    retry_status_forcelist=[500, 502, 503, 504],
    headers={"User-Agent": "SiftClient/1.0"},
)

# Create a REST client
client = RestClient(config)

# Make a GET request
response = client.get("/v1/users")

# Make a POST request
response = client.post(
    "/v1/users",
    json_data={"name": "John Doe", "email": "john.doe@example.com"},
)

# Make a PUT request
response = client.put(
    "/v1/users/123",
    json_data={"name": "Jane Doe"},
)

# Make a DELETE request
response = client.delete("/v1/users/123")

# Close the client when done
client.close()

# Or use the client as a context manager
with RestClient(config) as client:
    response = client.get("/v1/users")
```

### gRPC Service

The gRPC service provides a client for making gRPC API calls to Sift services.

```python
from sift_client.grpc.client import GrpcClient, GrpcConfig
from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub

# Create a gRPC client configuration
config = GrpcConfig(
    uri="api.sift.com",
    api_key="your-api-key",
    use_ssl=True,
    max_retries=3,
    retry_delay=1.0,
    retry_backoff_factor=2.0,
    timeout=30,
)

# Create a gRPC client
client = GrpcClient(config)

# Get a stub for the service
ping_stub = client.get_stub(PingServiceStub)

# Create a request
request = PingRequest(message="Hello, world!")

# Execute the request
response = client.execute(ping_stub.Ping, request)

# Close the client when done
client.close()

# Or use the client as a context manager
with GrpcClient(config) as client:
    ping_stub = client.get_stub(PingServiceStub)
    request = PingRequest(message="Hello, world!")
    response = client.execute(ping_stub.Ping, request)
```

## Error Handling

The library provides a set of exception classes for handling errors:

- `ClientError`: Base exception for client errors.
- `RequestError`: Exception raised when a request fails.
- `AuthenticationError`: Exception raised when authentication fails.
- `RateLimitError`: Exception raised when rate limits are exceeded.
- `ServerError`: Exception raised when the server returns an error.
- `TimeoutError`: Exception raised when a request times out.

Example of error handling:

```python
from sift_client.wrapper.utils import ClientError, AuthenticationError, ServerError
from sift_client.rest.client import RestClient, RestConfig

# Create a REST client configuration
config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
)

# Create a REST client
client = RestClient(config)

try:
    response = client.get("/v1/users")
except AuthenticationError as e:
    print(f"Authentication failed: {e.message}")
except ServerError as e:
    print(f"Server error: {e.message}")
except ClientError as e:
    print(f"Request failed: {e.message}")
```

## Retry Logic

The library provides retry logic for handling transient errors:

```python
from sift_client.wrapper.utils import retry
from sift_client.rest.client import RestClient, RestConfig

# Create a REST client configuration
config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
)

# Create a REST client
client = RestClient(config)

@retry(max_retries=3, retry_delay=1.0, backoff_factor=2.0)
def make_request():
    # Make a request that might fail
    return client.get("/v1/users")
```

## Advanced Usage

### Custom Headers

```python
from sift_client.rest.client import RestClient, RestConfig
from sift_client.grpc.client import GrpcClient, GrpcConfig
from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub

# REST client example
rest_config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
)
rest_client = RestClient(rest_config)
rest_response = rest_client.get(
    "/v1/users",
    headers={"X-Custom-Header": "value"},
)

# gRPC client example
grpc_config = GrpcConfig(
    uri="api.sift.com",
    api_key="your-api-key",
)
grpc_client = GrpcClient(grpc_config)
ping_stub = grpc_client.get_stub(PingServiceStub)
ping_request = PingRequest(message="Hello, world!")
grpc_response = grpc_client.execute(
    ping_stub.Ping,
    ping_request,
    metadata=[("x-custom-header", "value")],
)
```

### Timeout

```python
from sift_client.rest.client import RestClient, RestConfig
from sift_client.grpc.client import GrpcClient, GrpcConfig
from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub

# REST client example
rest_config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
)
rest_client = RestClient(rest_config)
rest_response = rest_client.get(
    "/v1/users",
    timeout=60,  # 60 seconds
)

# gRPC client example
grpc_config = GrpcConfig(
    uri="api.sift.com",
    api_key="your-api-key",
)
grpc_client = GrpcClient(grpc_config)
ping_stub = grpc_client.get_stub(PingServiceStub)
ping_request = PingRequest(message="Hello, world!")
grpc_response = grpc_client.execute(
    ping_stub.Ping,
    ping_request,
    timeout=60,  # 60 seconds
)
```

### Custom Retry Logic

```python
from sift_client.wrapper.utils import retry, ServerError, RateLimitError
from sift_client.rest.client import RestClient, RestConfig

# Create a REST client configuration
config = RestConfig(
    base_url="api.sift.com",
    api_key="your-api-key",
)

# Create a REST client
client = RestClient(config)

@retry(
    max_retries=5,
    retry_delay=2.0,
    backoff_factor=3.0,
    retry_on=[ServerError, RateLimitError],
)
def make_request():
    # Make a request that might fail
    return client.get("/v1/users")
```

## License

This library is licensed under the MIT License. See the LICENSE file for details.
