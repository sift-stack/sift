"""
This module is primarily concerned with configuring and initializing gRPC connections to the Sift API.

Example of establishing a connection to Sift's gRPC APi:

```python
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel

# Be sure not to include the url scheme i.e. 'https://' in the uri.
sift_channel_config = SiftChannelConfig(uri=SIFT_BASE_URI, apikey=SIFT_API_KEY)

with use_sift_channel(sift_channel_config) as channel:
    # Connect to Sift
```
"""
