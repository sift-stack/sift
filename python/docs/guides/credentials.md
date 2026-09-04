# Credentials and profiles

`SiftClient` finds its credentials the same way `sift-cli` does. If you already
run `sift-cli`, the Python client works with no arguments:

```python
from sift_client import SiftClient

client = SiftClient()
```

To use a named environment, give the profile name that you pass to
`sift-cli --profile`:

```python
client = SiftClient(profile="staging")

# The same thing, and easier to find in the docs:
client = SiftClient.from_profile("staging")
```

## The config file

`sift-cli` keeps one or more profiles in a `sift.toml` file in your user config
directory. Use the CLI to create and edit that file:

```bash
sift-cli config create
sift-cli config update --profile staging
sift-cli config where     # prints the path
```

The top-level table is the default profile. Each named table is one more
profile:

```toml
grpc_uri = "https://api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri  = "https://app.siftstack.com"
api_key  = "..."

[staging]
grpc_uri = "https://api.staging.siftstack.com"
rest_uri = "https://api.staging.siftstack.com"
app_uri  = "https://app.staging.siftstack.com"
api_key  = "..."

[localdev]
grpc_uri = "http://localhost:50051"
rest_uri = "http://localhost:8080"
api_key  = "local"
```

A profile does not inherit from the default profile. If `[staging]` has no
`api_key`, the client reports an error. It does not fall back to the default
profile's key, because that key can point your tests at a different
environment.

Use an `http://` scheme for a plaintext endpoint, as `[localdev]` does above.
The client reads the scheme to select TLS or plaintext.

Older files spell the key `apikey`. That spelling is still valid, so you do not
need to migrate. `api_key` is canonical, matches the rest of the Sift API, and
is what `sift-cli config update` writes. If a profile holds both keys, the
client uses `api_key`.

## Resolution order

Highest precedence first:

1. Arguments that you pass to `SiftClient`, one field at a time.
2. The fields of a profile that you name with `profile=`.
3. The environment variables `SIFT_API_KEY`, `SIFT_GRPC_URI`, `SIFT_REST_URI`,
   and `SIFT_APP_URL`.
4. The fields of the profile that `SIFT_PROFILE` names.
5. The default (top-level) table of the config file.

A profile that you name in code outranks the environment.
`SiftClient(profile="prod")` therefore reaches production even in a shell that
points somewhere else. `SIFT_PROFILE` does not outrank the environment, so CI
can select a profile for its endpoints and still inject the API key:

```bash
export SIFT_PROFILE=staging      # endpoints from the staging profile
export SIFT_API_KEY="$CI_SECRET" # key from the secret store
pytest
```

The client reads one profile at most. If you set both `profile=` and
`SIFT_PROFILE`, the client uses `profile=` and ignores the other profile.

## Where the client looks for the file

1. If `SIFT_CONFIG_FILE` is set, the client uses that path.
2. If not, the client uses your user config directory:
   `$XDG_CONFIG_HOME/sift.toml` (or `~/.config/sift.toml`) on Linux,
   `~/Library/Application Support/sift.toml` on macOS, and
   `%APPDATA%\sift.toml` on Windows.

The client does not search the current working directory. A `sift.toml` file in
a repository that you cloned therefore cannot supply an API key.

## Check what a client resolved

`credential_sources` reports the layer that supplied each value:

```python
client = SiftClient(profile="staging")
client.profile               # 'staging'
client.credential_sources    # {'grpc_url': 'profile:staging', 'api_key': 'env', ...}
```

Each value is `arg`, `profile:<name>`, `env`, `default`, or `unset`. Both
properties are `None` if you build the client from an explicit
`connection_config`, because that path does not resolve credentials.

## Pass credentials directly

Explicit arguments and `connection_config` work as before. Use them if the
credentials come from a source that the resolver does not read, such as a
secrets manager:

```python
client = SiftClient(
    api_key="...",
    grpc_url="https://api.siftstack.com",
    rest_url="https://api.siftstack.com",
)
```

## Errors

If `SiftClient` cannot resolve the API key or either URL, it raises
`SiftCredentialsError`, a subclass of `ValueError`. The message names the
missing variables and the file and profile that the client read. It also lists
the profiles in that file and gives the `sift-cli` command that sets the
missing values.

## Use with pytest

The pytest plugin reads the same profiles. For the plugin's own settings, see
[Configuration & Defaults](pytest_plugin/configuration.md).

The plugin uses a different precedence order. Its environment variables,
`--sift-*` flags, and `sift_grpc_uri` / `sift_rest_uri` ini keys all outrank the
profile. The profile supplies only the values that they leave unset, so a key
that CI injects stays in effect.

```bash
pytest --sift-profile staging
```

```toml
# pyproject.toml
[tool.pytest.ini_options]
sift_profile = "staging"
```
