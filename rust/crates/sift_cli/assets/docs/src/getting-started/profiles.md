# Profiles

A profile is a named set of credentials in your `sift.toml`. Profiles let you
switch between Sift environments, for example production and staging, without
re-editing your config.

## The default profile

When you do not name a profile, settings are read from and written to the top
level of the config file. This is the `default` profile:

```toml
grpc_uri = "https://api.siftstack.com"
rest_uri = "https://api.siftstack.com"
apikey = "sift_prod_..."
```

## Adding a named profile

Pass `--profile <name>` to `config update`. The profile is created as its own
TOML table:

```sh
sift-cli config update --profile mission \
  --grpc-uri https://api.staging.example.com \
  --rest-uri https://api.staging.example.com \
  --api-key "$SIFT_STAGING_API_KEY"
```

The resulting file:

```toml
grpc_uri = "https://api.siftstack.com"
rest_uri = "https://api.siftstack.com"
apikey = "sift_prod_..."

[staging]
grpc_uri = "https://api.staging.example.com"
rest_uri = "https://api.staging.example.com"
apikey = "sift_staging_..."
```

The interactive flow also supports profiles. When prompted for a profile name,
enter `staging` instead of leaving it blank:

```sh
sift-cli config update --interactive
```

## Using a profile

`--profile` is a global flag, so it goes on any command that talks to Sift:

```sh
sift-cli --profile mission ping
sift-cli --profile mission import csv ./telemetry.csv --asset rover-1
```

Omit `--profile` to use the default profile.

## Agent integration profile

Install the Sift MCP integration for every detected AI coding client with the
same named profile:

```sh
sift-cli agent install --profile mission
```

An ordinary `sift-cli agent update` preserves the installed profile. Switch
every client together with `sift-cli agent update --profile <name>`, or return
all clients to the default profile with:

```sh
sift-cli agent update --default-profile
```

`sift-cli agent doctor` reports the profile in each registration and treats
mixed profiles as an error.
