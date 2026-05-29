# Exporting Data

`sift-cli export` pulls data back out of Sift. You can export by **run** or by
**asset**, choosing which channels to include and the output format.

```
sift-cli export run   (--name | --run-id | --client-key) --output <FILE> --format <FORMAT> [OPTIONS]
sift-cli export asset <ASSET> --output <FILE> --format <FORMAT> --start <RFC3339> --stop <RFC3339> [OPTIONS]
```

## Output formats

`--format` (`-f`) accepts: `csv`, `parquet`, and `sun`.

## Exporting a run

Identify the run by name, ID, or client key (choose one):

```sh
sift-cli export run --name "2026-05-29 field test" \
  --output ./field-test.csv \
  --format csv
```

```sh
sift-cli export run --run-id 0123abcd-... \
  --output ./field-test.parquet \
  --format parquet
```

A run already carries its own time bounds, so `--start` and `--stop` are
optional for run exports.

## Exporting an asset

Asset exports require an explicit time range with `--start` and `--stop` in
RFC 3339:

```sh
sift-cli export asset rover-1 \
  --output ./rover-day.parquet \
  --format parquet \
  --start 2026-05-29T00:00:00Z \
  --stop 2026-05-30T00:00:00Z
```

## Choosing channels

Without channel filters, all channels are exported. Narrow the set by name, ID,
or regular expression. The channel flags are repeatable and can be combined.

```sh
sift-cli export asset rover-1 \
  --output ./speed-and-temp.csv \
  --format csv \
  --start 2026-05-29T00:00:00Z \
  --stop 2026-05-30T00:00:00Z \
  --channel speed \
  --channel temperature
```

Match a family of channels with a regex:

```sh
sift-cli export asset rover-1 \
  --output ./motors.csv \
  --format csv \
  --start 2026-05-29T00:00:00Z \
  --stop 2026-05-30T00:00:00Z \
  --channel-regex '^motor\.'
```

Calculated channels have their own flags: `--calculated-channel`,
`--calculated-channel-id`, and `--calculated-channel-regex`.

## Options

| Flag                                | Applies to   | Description                                                |
| ----------------------------------- | ------------ | --------------------------------------------------------- |
| `--name`, `-n`                      | run          | Run name (choose one identifier).                         |
| `--run-id`, `-r`                    | run          | Run ID (choose one identifier).                           |
| `--client-key`, `-k`                | run          | Run client key (choose one identifier).                   |
| `<ASSET>`                           | asset        | Asset name (positional).                                  |
| `--output`, `-o`                    | both         | Output file path (required).                              |
| `--format`, `-f`                    | both         | `csv`, `parquet`, or `sun` (required).                    |
| `--start`                           | both         | RFC 3339 start time (required for asset exports).         |
| `--stop`                            | both         | RFC 3339 stop time (required for asset exports).          |
| `--channel`, `-c`                   | both         | Channel name to include; repeatable.                      |
| `--channel-id`                      | both         | Channel ID to include; repeatable.                        |
| `--channel-regex`, `-x`             | both         | Regex to filter channels.                                 |
| `--calculated-channel`              | both         | Calculated channel name; repeatable.                      |
| `--calculated-channel-id`           | both         | Calculated channel ID; repeatable.                        |
| `--calculated-channel-regex`        | both         | Regex to filter calculated channels.                      |
