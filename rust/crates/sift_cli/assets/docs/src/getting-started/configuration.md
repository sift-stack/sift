# Configuration

Before the CLI can talk to Sift it needs three things:

| Field      | Description                                  | Example                        |
| ---------- | -------------------------------------------- | ------------------------------ |
| `grpc_uri` | Base gRPC endpoint for Sift                  | `https://api.siftstack.com`    |
| `rest_uri` | Base REST endpoint for Sift                  | `https://api.siftstack.com`    |
| `apikey`   | Your Sift API key                            | `sift_...`                     |

For Sift Cloud, both URIs are `https://api.siftstack.com`. For self-hosted or
non-cloud environments, use the endpoints provided by your administrator (and
see [Disabling TLS](#disabling-tls) below if they are not served over TLS).

You can generate an API key from the Sift web app under your account settings.

## The config file

Settings live in a TOML file named `sift.toml` inside your OS config directory:

| Platform | Location                                            |
| -------- | --------------------------------------------------- |
| macOS    | `~/Library/Application Support/sift.toml`            |
| Linux    | `~/.config/sift.toml`                                |
| Windows  | `%APPDATA%\sift.toml`                                |

Find the exact path on your machine:

```sh
sift-cli config where
```

## Bootstrapping your credentials

The fastest way to get set up is the interactive flow. First create the file,
then fill it in:

```sh
sift-cli config create
sift-cli config update --interactive
```

The interactive prompt walks you through the profile name, both URIs, and your
API key, then shows the result for confirmation before writing it. Leaving the
profile blank configures the `default` profile.

### Non-interactive setup

To configure everything in one command (useful for scripts and CI), pass the
values as flags:

```sh
sift-cli config update \
  --grpc-uri https://api.siftstack.com \
  --rest-uri https://api.siftstack.com \
  --api-key "$SIFT_API_KEY"
```

Short forms: `-g` (grpc), `-r` (rest), `-k` (api key). Any field you omit is
left untouched, so you can update one value at a time:

```sh
sift-cli config update --api-key "$SIFT_API_KEY"
```

## Inspecting the config

```sh
sift-cli config show     # print the current config file
sift-cli config where    # print the path to the config file
```

A configured `default` profile looks like this:

```toml
grpc_uri = "https://api.siftstack.com"
rest_uri = "https://api.siftstack.com"
apikey = "sift_..."
```

## Disabling TLS

For non-cloud Sift environments that are not served over TLS, pass the global
`--disable-tls` flag on any command that talks to Sift:

```sh
sift-cli --disable-tls ping
```

## Next steps

- Manage more than one environment with [Profiles](./profiles.md).
- Confirm everything works in [Verifying Your Setup](./verifying.md).
