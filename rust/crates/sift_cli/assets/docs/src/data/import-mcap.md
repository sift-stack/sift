# Importing MCAP

```
sift-cli import mcap <PATH> --asset <NAME> [OPTIONS]
```

MCAP (`.mcap`) is a container format for recorded robotics and vehicle
telemetry. The CLI imports every decodable topic in the file, one channel per
flattened field, named `<topic>.<field path>` (for example
`/imu/data.orientation.x`).

Only ROS 2 topics are supported: `cdr` messages carrying `ros2msg` schemas.
Chunks may be uncompressed or compressed with zstd or lz4.

## Quick start

```sh
sift-cli import mcap ./recording.mcap --asset rover-1
```

Attach to a run and wait for processing:

```sh
sift-cli import mcap ./recording.mcap \
  --asset rover-1 \
  --run "field test 2026-08-14" \
  --wait
```

## Previewing the detected channels

`--preview` (`-p`) parses the file locally and prints the channels the import
creates, without uploading anything:

```sh
sift-cli import mcap ./recording.mcap --asset rover-1 --preview
```

Topics the client cannot decode are listed as warnings with the reason. The
preview reads schema and channel records only, so a topic appears even when it
logged no messages.

## How fields become channels

Each topic's message type is flattened into leaf fields:

| Message field                       | Channels                                      |
| ----------------------------------- | --------------------------------------------- |
| A scalar (`float64 x`)              | One channel.                                  |
| A nested message                    | One channel per leaf, joined by `.`           |
| A fixed-size array (`float64[9] c`) | One channel per element, `c[0]` … `c[8]`      |
| A dynamic or bounded array          | One variable-cardinality field (see below).   |
| `builtin_interfaces/Time`, `Duration` | One `int64` nanosecond channel.             |

Narrow integers widen to 32-bit, matching the other import types. `byte` and
`char` import as `uint32`, which is how ROS 2 defines them.

## Variable-cardinality fields

Dynamic (`float64[]`) and bounded (`uint8[<=4]`) arrays carry a different
number of values per message, so they import whole rather than one channel per
element. `--complex-types-import-mode` decides what each becomes:

| Value             | Result                                                          |
| ----------------- | --------------------------------------------------------------- |
| `both` (default)  | Arrow IPC bytes under the field's name **and** a JSON string under `<name>.json`. |
| `bytes`           | Arrow IPC bytes only.                                           |
| `string`          | A JSON string under `<name>.json` only.                         |
| `ignore`          | Not imported.                                                   |

```sh
sift-cli import mcap ./recording.mcap \
  --asset rover-1 \
  --complex-types-import-mode string
```

## Anchoring timestamps

MCAP log times are Unix-epoch nanoseconds, so no anchoring is needed for a
normal recording. For a log written on a different epoch, pass the UTC time of
log start; each message's log time is then read as nanoseconds elapsed from
that start:

```sh
sift-cli import mcap ./recording.mcap \
  --asset rover-1 \
  --relative-start-time 2026-08-14T15:04:05Z
```

## Importing metadata records

MCAP files can carry named metadata records of key-value pairs. Import
selected records as run metadata with `--metadata-record`, which requires
`--run` or `--run-id`:

```sh
sift-cli import mcap ./recording.mcap \
  --asset rover-1 \
  --run "field test 2026-08-14" \
  --metadata-record hardware \
  --metadata-record software
```

Every key of a named record is stored as `<record>.<key>`.

## Recoverable parse errors

A recording may end mid-record after a power loss, or carry topics the
importer cannot decode. By default the import fails on such errors; pass
`--parse-error-policy ignore-error` to import what decoded instead, with the
skipped topics and records reported as warnings on the import:

```sh
sift-cli import mcap ./recording.mcap \
  --asset rover-1 \
  --parse-error-policy ignore-error
```

## Options

| Flag                            | Description                                                          |
| ------------------------------- | -------------------------------------------------------------------- |
| `--asset`, `-a`                 | Asset the data belongs to (required).                                |
| `--run`, `-r`                   | Run name to associate with the import.                               |
| `--run-id`                      | Attach to an existing run by ID (mutually exclusive with `--run`).    |
| `--relative-start-time`, `-s`   | UTC time of log start, for logs on a non-Unix epoch.                 |
| `--metadata-record`             | Metadata record to import as run metadata; repeatable.               |
| `--parse-error-policy`          | Handling for recoverable parse errors (default `fail-on-error`).      |
| `--complex-types-import-mode`   | Handling for variable-cardinality fields (default `both`).            |
| `--preview`, `-p`               | Print the detected channels without uploading.                        |
| `--wait`, `-w`                  | Block until Sift finishes processing.                                 |
