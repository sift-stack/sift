# Importing CSV

```
sift-cli import csv <PATH> --asset <NAME> [OPTIONS]
```

By default, every non-time column becomes a channel and its type is inferred as
either `string` or `double`. The first row is treated as the header and data
starts on the second row.

## Quick start

Import a file where column 1 is an RFC 3339 timestamp and the remaining columns
are channels:

```sh
sift-cli import csv ./telemetry.csv --asset rover-1
```

Preview the inferred schema first:

```sh
sift-cli import csv ./telemetry.csv --asset rover-1 --preview
```

Attach the data to a run and wait for processing to finish:

```sh
sift-cli import csv ./telemetry.csv \
  --asset rover-1 \
  --run "2026-05-29 field test" \
  --wait
```

## Pointing at the right rows and time column

If your file has metadata rows above the header, or the timestamp is not the
first column, set them explicitly (all indices are 1-based):

```sh
sift-cli import csv ./telemetry.csv \
  --asset rover-1 \
  --header-row 3 \
  --first-data-row 5 \
  --time-column 2
```

## Relative timestamps

When the time column holds offsets rather than absolute timestamps, declare the
format and anchor it with a start time:

```sh
sift-cli import csv ./telemetry.csv \
  --asset rover-1 \
  --time-format relative-seconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

## Overriding channel types and metadata

Use `--channel-column` to target a 1-based column, then provide a matching
`--data-type`, `--unit`, and/or `--description` in the same order. Pass `infer`
as the data type when you only want to set the unit or description.

```sh
sift-cli import csv ./telemetry.csv \
  --asset rover-1 \
  --channel-column 2 --data-type double --unit "m/s" --description "ground speed" \
  --channel-column 3 --data-type infer --unit "C"
```

## Enum and bit-field channels

Enum channels map integer keys to names with `--enum-config`, using
`<key,name>` pairs separated by `|`:

```sh
sift-cli import csv ./states.csv \
  --asset rover-1 \
  --channel-column 2 --data-type enum \
  --enum-config "0,idle|1,driving|2,charging"
```

Bit-field channels decode bits with `--bit-field-config`, using
`<name,index,length>` triplets separated by `|`:

```sh
sift-cli import csv ./flags.csv \
  --asset rover-1 \
  --channel-column 2 --data-type bit-field \
  --bit-field-config "12v,0,4|led,4,4"
```

## Options

| Flag                              | Description                                                     |
| --------------------------------- | --------------------------------------------------------------- |
| `--asset`, `-a`                   | Asset the data belongs to (required).                           |
| `--run`, `-r`                     | Run name to associate with the import.                          |
| `--header-row`                    | 1-based row holding column headers (default `1`).               |
| `--first-data-row`                | 1-based row where data starts (default `2`).                    |
| `--time-column`, `-t`             | 1-based index of the time column (default `1`).                 |
| `--time-format`, `-f`             | Time format (default `absolute-rfc3339`).                       |
| `--relative-start-time`, `-s`     | RFC 3339 start time for relative time formats.                  |
| `--channel-column`, `-c`          | 1-based column to override; repeatable.                         |
| `--data-type`, `-d`               | Data type per `--channel-column`; repeatable.                   |
| `--unit`, `-u`                    | Unit per `--channel-column`; repeatable.                        |
| `--description`, `-n`             | Description per `--channel-column`; repeatable.                 |
| `--enum-config`, `-e`             | Enum `<key,name>` pairs for enum channels.                      |
| `--bit-field-config`, `-b`        | Bit-field `<name,index,length>` triplets.                       |
| `--preview`, `-p`                 | Print the parsed schema without uploading.                      |
| `--wait`, `-w`                    | Block until Sift finishes processing.                          |
