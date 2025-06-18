# sift_connect

[![Crates.io](https://img.shields.io/crates/v/sift_connect.svg)](https://crates.io/crates/sift_connect)
[![docs.rs](https://img.shields.io/docsrs/sift_connect)](https://docs.rs/sift_connect/latest/sift_connect/)

`sift_connect` is a convenience crate that makes it simple to create a gRPC connection to
 Sift. Most users won't need to install this crate as it's re-exported in other crates. The
 following is an example of how to create a gRPC connection to Sift using sane defaults.

 ```rust
 use sift_connect::{Credentials, SiftChannelBuilder};
 use std::env;

 let credentials = Credentials::Config {
     uri: env::var("SIFT_URI").unwrap(),
     apikey: env::var("SIFT_API_KEY").unwrap(),
 };

 let grpc_conn = SiftChannelBuilder::new(credentials)
     .build()
     .unwrap();
 ```

 ## Configuration File

Credentials can also be loaded from a `sift.toml` file stored in a the user's [local config
directory](https://docs.rs/dirs/6.0.0/dirs/fn.config_local_dir.html).

The following is an example of a valid `sift.toml` file:

```toml
uri = "http://example-sift-api.com"
apikey = "example-sift-api-key"

[mission]
uri = "http://example-sift-api.com"
apikey = "my-other-sift-api-key"
```

The top-level TOML table is considered the default profile, with the `mission` table being a
named profile. To reference the default profile:

```rust
use sift_connect::{Credentials, SiftChannelBuilder};
use std::env;

let credentials = Credentials::Profile(None);

let grpc_conn = SiftChannelBuilder::new(credentials)
    .build()
    .unwrap();
```

To reference the `mission` profile:

```rust
use sift_connect::{Credentials, SiftChannelBuilder};
use std::env;

let credentials = Credentials::Profile(Some("mission".into()));

let grpc_conn = SiftChannelBuilder::new(credentials)
    .build()
    .unwrap();
```
