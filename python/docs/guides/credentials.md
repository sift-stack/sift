# Credentials and profiles

`SiftClient` finds its credentials the same way `sift-cli` does. If you already
run `sift-cli`, the Python client works with no arguments:

```python
from sift_client import SiftClient

client = SiftClient()
```

To use a named environment, give it a profile name, the same one you pass to
`sift-cli --profile`:

```python
client = SiftClient(profile="staging")

# Equivalent, and easier to find in the docs:
client = SiftClient.from_profile("staging")
```

## The config file

`sift-cli` keeps one or more profiles in a `sift.toml` under your user config
directory. Create and edit it with the CLI rather than by hand:

```bash
sift-cli config create
sift-cli config update --profile staging
sift-cli config where     # prints the path
```

The file looks like this. The top-level table is the default profile; each
named table is a profile:

```toml
grpc_uri = "https://api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri  = "https://app.siftstack.com"
apikey   = "..."

[staging]
grpc_uri = "https://api.staging.siftstack.com"
rest_uri = "https://api.staging.siftstack.com"
app_uri  = "https://app.staging.siftstack.com"
apikey   = "..."

[localdev]
grpc_uri = "http://localhost:50051"
rest_uri = "http://localhost:8080"
apikey   = "local"
```

A profile does not inherit from the default profile. If `[staging]` has no
`apikey`, that is an error rather than a silent fall back to the default
profile's key, which would otherwise point your tests at one environment using
another environment's credentials.

Use an `http://` scheme for a plaintext endpoint, as `[localdev]` does above.
The client reads the scheme to decide whether to use TLS.

## Resolution order

Highest precedence first:

1. Arguments you pass to `SiftClient`, per field.
2. The fields of a profile named by `profile=`.
3. The environment variables `SIFT_API_KEY`, `SIFT_GRPC_URI`, `SIFT_REST_URI`,
   and `SIFT_APP_URL`.
4. The fields of the profile named by the `SIFT_PROFILE` environment variable.
5. The default (top-level) table of the config file.

Naming a profile in code outranks the environment, so `SiftClient(profile="prod")`
still reaches production in a shell that was pointed somewhere else.
`SIFT_PROFILE` does not, so CI can select a profile for its endpoints and still
inject the API key through `SIFT_API_KEY`:

```bash
export SIFT_PROFILE=staging      # endpoints from the staging profile
export SIFT_API_KEY="$CI_SECRET" # key from the secret store
pytest
```

Only one profile is ever read. If both `profile=` and `SIFT_PROFILE` are set,
the argument wins and the other profile is ignored entirely.

## Where the file is looked for

1. `SIFT_CONFIG_FILE`, when set, is used directly.
2. Otherwise the user config directory: `$XDG_CONFIG_HOME/sift.toml` (or
   `~/.config/sift.toml`) on Linux, `~/Library/Application Support/sift.toml`
   on macOS, and `%APPDATA%\sift.toml` on Windows.

The current working directory is not searched, so a `sift.toml` committed to a
repository you cloned cannot supply an API key.

## Checking what a client resolved

`credential_sources` reports which layer supplied each value, which is usually
faster than re-deriving the precedence by hand:

```python
client = SiftClient(profile="staging")
client.profile               # 'staging'
client.credential_sources    # {'grpc_url': 'profile:staging', 'api_key': 'env', ...}
```

Each value is `arg`, `profile:<name>`, `env`, `default`, or `unset`. Both are
`None` when the client was built from an explicit `connection_config`, which
bypasses resolution entirely.

## Passing credentials directly

Explicit arguments and `connection_config` work exactly as before. Use them
when credentials come from somewhere the resolver does not know about, such as
a secrets manager:

```python
client = SiftClient(
    api_key="...",
    grpc_url="https://api.siftstack.com",
    rest_url="https://api.siftstack.com",
)
```

## Errors

When the API key or either URL cannot be resolved, `SiftClient` raises
`SiftCredentialsError`, which subclasses `ValueError`. The message names the
missing variables, the file and profile it looked in, the profiles that file
defines, and the `sift-cli` command that sets them.

## Use with pytest

The pytest plugin reads the same profiles. See
[Configuration & Defaults](pytest_plugin/configuration.md) for the plugin's own
settings, and note one difference: in the plugin, the plugin's existing
surfaces (environment variables, `--sift-*` flags, and the
`sift_grpc_uri` / `sift_rest_uri` ini keys) all outrank the profile, which
fills in whatever they leave unset. That keeps CI-injected values authoritative.

```bash
pytest --sift-profile staging
```

```toml
# pyproject.toml
[tool.pytest.ini_options]
sift_profile = "staging"
```
