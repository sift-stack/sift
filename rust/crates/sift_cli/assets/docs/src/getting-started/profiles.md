# Profiles

A profile is a named set of credentials in your `sift.toml`. Profiles let you
switch between Sift environments, for example production and staging, without
re-editing your config.

## The default profile

When you do not name a profile, settings are read from and written to the top
level of the config file. This is the `default` profile. `--profile default`
also selects it:

```toml
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
api_key = "sift_prod_..."
```

## Adding a named profile

In a terminal, pass the new name to `config create`. It prompts for each value
of the new profile:

```sh
sift-cli --profile staging config create
```

Without `--profile`, `config create` asks for a name when the `default` profile
is already set up. If the profile already exists, the command tells you to use
`config update` instead.

For scripts, pass `--profile <name>` and field flags to `config update`. A name
that does not exist yet is created:

```sh
sift-cli --profile staging config update \
  --grpc-uri https://grpc-api.staging.example.com \
  --rest-uri https://api.staging.example.com \
  --app-uri https://sift.staging.example.com \
  --api-key "$SIFT_STAGING_API_KEY"
```

The resulting file:

```toml
grpc_uri = "https://grpc-api.siftstack.com"
rest_uri = "https://api.siftstack.com"
app_uri = "https://app.siftstack.com"
api_key = "sift_prod_..."

[staging]
grpc_uri = "https://grpc-api.staging.example.com"
rest_uri = "https://api.staging.example.com"
app_uri = "https://sift.staging.example.com"
api_key = "sift_staging_..."
```

Each profile is stored as its own TOML table.

## Editing a named profile

Run `config update` without field flags. The command lists the profiles and
asks which one to edit:

```text
Select a profile to edit:
  1) default
  2) staging
Profile [1]: 2
```

Enter a number or a profile name. To skip the list, name the profile:

```sh
sift-cli --profile staging config update
```

## Using a profile

`--profile` is a global flag, so it goes on any command that talks to Sift:

```sh
sift-cli --profile staging ping
sift-cli --profile staging import csv ./telemetry.csv --asset rover-1
```

Omit `--profile` to use the default profile.
