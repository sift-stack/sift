# Importing TDMS

```
sift-cli import tdms <PATH> --asset <NAME> [OPTIONS]
```

TDMS is the National Instruments measurement format. The CLI reads the file's
channels and their timing information directly, so in the common case you only
need to name the asset.

## Quick start

```sh
sift-cli import tdms ./measurement.tdms --asset test-rig-3
```

Attach to a run and wait for processing:

```sh
sift-cli import tdms ./measurement.tdms \
  --asset test-rig-3 \
  --run "burn-in 2026-05-29" \
  --wait
```

## Channels with missing timing

When a channel lacks timing metadata, choose how the CLI should respond with
`--fallback-method` (default `fail-on-error`). Run the command with `--help` to
see the available methods for your version.

```sh
sift-cli import tdms ./measurement.tdms \
  --asset test-rig-3 \
  --fallback-method fail-on-error
```

For channels that use a non-standard time channel, set the time format and a
relative start time:

```sh
sift-cli import tdms ./measurement.tdms \
  --asset test-rig-3 \
  --time-format relative-seconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

## Importing file properties as metadata

Pass `--import-file-properties` to copy the TDMS file's properties onto the run
as metadata:

```sh
sift-cli import tdms ./measurement.tdms \
  --asset test-rig-3 \
  --run "burn-in 2026-05-29" \
  --import-file-properties
```

## Options

| Flag                          | Description                                                       |
| ----------------------------- | ---------------------------------------------------------------- |
| `--asset`, `-a`               | Asset the data belongs to (required).                            |
| `--run`, `-r`                 | Run name to associate with the import.                           |
| `--run-id`                    | Attach to an existing run by ID (takes precedence over `--run`). |
| `--start-time-override`       | Override the start time.                                         |
| `--fallback-method`, `-f`     | Handling for channels with missing timing (default `fail-on-error`). |
| `--time-format`               | Time format for channels not using the TDMS timestamp type.      |
| `--relative-start-time`, `-s` | Relative start time for a non-standard time channel.             |
| `--import-file-properties`    | Import TDMS file properties to the run as metadata.              |
| `--preview`, `-p`             | Print the parsed schema without uploading.                       |
| `--wait`, `-w`                | Block until Sift finishes processing.                           |
