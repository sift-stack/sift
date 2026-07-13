# Importing HDF5

```
sift-cli import hdf5 <SCHEMA> <PATH> --asset <NAME> --time-format <FORMAT> [OPTIONS]
```

where `<SCHEMA>` is `one-d`, `two-d`, or `compound`.

HDF5 files vary widely in layout, so you tell the CLI how the file is organized
by choosing a schema subcommand. Supported channel types across all schemas
are: `bool`, `int8/16/32/64`, `uint8/16/32/64`, `float32`, and `float64`.
Datasets with other types produce a client-side error.

## Schemas

| Subcommand | Layout                                                        |
| ---------- | ------------------------------------------------------------- |
| `one-d`    | One dataset per channel, plus a one-dimensional time dataset. |
| `two-d`    | A two-dimensional dataset where each column is a channel.     |
| `compound` | A compound (record) dataset where each field is a channel.    |

## one-d

Each channel is its own dataset and there is a separate time dataset. The CLI
auto-detects common time dataset names (`time`, `timestamp`, `timestamps`,
`ts`). If yours differs, set it with `--time-name`:

```sh
sift-cli import hdf5 one-d ./capture.h5 \
  --asset sensor-array \
  --time-format absolute-unix-nanoseconds \
  --time-name epoch_ns
```

## two-d

Channels are columns of a single 2-D dataset. Identify the time column by index
with `--time-index` (defaults to `0`):

```sh
sift-cli import hdf5 two-d ./capture.h5 \
  --asset sensor-array \
  --time-format absolute-unix-microseconds \
  --time-index 0
```

## compound

Channels are fields of a compound dataset. Identify the time field by name with
`--time-field`, or by position with `--time-index` (defaults to `0`). The two
are mutually exclusive:

```sh
sift-cli import hdf5 compound ./capture.h5 \
  --asset sensor-array \
  --time-format absolute-rfc3339 \
  --time-field t
```

## Relative timestamps

For any schema, a relative time format needs an anchor:

```sh
sift-cli import hdf5 one-d ./capture.h5 \
  --asset sensor-array \
  --time-format relative-milliseconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

## Options

| Flag                          | Description                                                              |
| ----------------------------- | ------------------------------------------------------------------------ |
| `--asset`, `-a`               | Asset the data belongs to (required).                                    |
| `--time-format`               | Time format used by the time dataset/column (required).                  |
| `--run`, `-r`                 | Run name to associate with the import.                                   |
| `--run-id`                    | Attach to an existing run by ID (mutually exclusive with `--run`).       |
| `--relative-start-time`, `-s` | RFC 3339 start time for relative time formats.                           |
| `--time-index`                | (`two-d`, `compound`) Index of the time column/field. Default `0`.       |
| `--time-field`                | (`compound`) Name of the time field.                                     |
| `--time-name`                 | (`one-d`) Name of the time dataset when it is not auto-detected.         |
| `--preview`, `-p`             | Print the parsed schema without uploading.                               |
| `--wait`, `-w`                | Block until Sift finishes processing.                                    |

`--time-index`, `--time-field`, and `--time-name` are specific to their schema
subcommand — use the one that matches yours.
