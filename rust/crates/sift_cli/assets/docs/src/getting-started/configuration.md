# Configuration

Each CLI profile needs four values:

| Field      | Description                                  | Example                          |
| ---------- | -------------------------------------------- | -------------------------------- |
| `grpc_uri` | Base gRPC endpoint for Sift                  | `https://grpc-api.siftstack.com` |
| `rest_uri` | Base REST endpoint for Sift                  | `https://api.siftstack.com`      |
| `app_uri`  | Required web app origin from your browser    | `https://app.siftstack.com`      |
| `api_key`  | Your Sift API key                            | `sift_...`                       |

For Sift Cloud, use the gRPC and REST values in the table. For other
environments, use the endpoints from your administrator. See
[Disabling TLS](#disabling-tls) if the endpoints do not use TLS.

The config command can infer `app_uri` for PubCloud and GovCloud Sift. For any
other domain, open the Sift web app and copy its URL origin. Keep the scheme and
host. Do not include a page path, query, or fragment.

A profile without a usable `app_uri` is incomplete. API commands can continue
so users can recover an old profile. Imports omit their web link. MCP URL tools
need `app_uri`. `sift-cli mcp` does not expose tools for the incomplete profile.
Its tool-list error includes the repair command, so the MCP client can show the
reason. `sift-cli agent doctor` treats a missing or unusable value as an error.

You can generate an API key from the Sift web app under your account settings.

## The config file

Settings live in a TOML file named `sift.toml` inside your OS config directory:

| Platform | Location                                            |
| -------- | --------------------------------------------------- |
| macOS    | `~/Library/Application Support/sift.toml`           |
| Linux    | `~/.config/sift.toml`                               |
| Windows  | `%APPDATA%\sift.toml`                               |

Find the exact path on your machine:

```sh
sift-cli config where
```

## Bootstrapping your credentials

Run `config create` in a terminal. It creates the config file and prompts for
the values of the `default` profile:

```sh
sift-cli config create
```

```text
Creating a config file at '~/Library/Application Support/sift.toml'.

Editing profile 'default'. Enter a value for each field.

  gRPC URL: https://grpc-api.siftstack.com
  REST URL: https://api.siftstack.com
  API key:

  App URL
    suggested: https://app.siftstack.com (leave blank to use it)
    new:

Changes to 'default':
  grpc_uri  (unset)  ->  https://grpc-api.siftstack.com
  rest_uri  (unset)  ->  https://api.siftstack.com
  api_key   (unset)  ->  ****a91f
  app_uri   (unset)  ->  https://app.siftstack.com
Apply? [Y/n]:
Saved profile 'default'.
Run 'sift-cli ping' to check the connection.
```

The terminal does not show the API key while you type it. The change list shows
only its last four characters. The command writes nothing until you confirm.

### Editing a profile

Run `config update` without field flags to edit a profile interactively:

```sh
sift-cli config update
```

If the file has named profiles, the command first asks which profile to edit.
Use `--profile <name>` to skip that question. Each field shows its current
value:

```text
Editing profile 'staging'.
To change a field, type a new value and press Enter. To keep the current value, leave it blank and press Enter.

  gRPC URL
    current: https://grpc-api.staging.example.net
    new:
```

The command lists only the fields you changed. If nothing changed, it writes
nothing.

`config update` edits existing profiles only. If the config file or the named
profile does not exist, the command stops and tells you to run `config create`.

### Non-interactive setup

Outside a terminal, `config create` only creates an empty config file. To set
the values in one command (for scripts and CI), pass them as flags:

```sh
sift-cli config create
sift-cli config update \
  --grpc-uri https://grpc-api.siftstack.com \
  --rest-uri https://api.siftstack.com \
  --app-uri https://app.siftstack.com \
  --api-key "$SIFT_API_KEY"
```

Short forms are `-g` for gRPC, `-r` for REST, and `-k` for the API key.
Omitted fields stay unchanged. The command prints the fields that changed. You
can update one value at a time:

```sh
sift-cli config update --api-key "$SIFT_API_KEY"
```

Without field flags outside a terminal, `config update` exits with status 2,
because it cannot prompt.

## Inspecting the config

```sh
sift-cli config show     # print the current config file
sift-cli config where    # print the path to the config file
```

A configured `default` profile looks like this:

```toml
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
api_key = "sift_..."
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
