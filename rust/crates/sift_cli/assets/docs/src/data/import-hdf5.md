# Importing HDF5

```
sift-cli import hdf5 <PATH> --asset <NAME> --schema <SCHEMA> [OPTIONS]
```

HDF5 files vary widely in layout, so you tell the CLI how the file is organized
with `--schema`. Supported channel types across all schemas are: `bool`,
`int8/16/32/64`, `uint8/16/32/64`, `float32`, and `float64`. Datasets with other
types produce a client-side error.

## Schemas

| Schema     | Layout                                                                  |
| ---------- | ---------------------------------------------------------------------- |
| `one-d`    | One dataset per channel, plus a one-dimensional time dataset.          |
| `two-d`    | A two-dimensional dataset where each column is a channel.              |
| `compound` | A compound (record) dataset where each field is a channel.            |

## one-d

Each channel is its own dataset and there is a separate time dataset. The CLI
auto-detects common time dataset names (`time`, `timestamp`, `timestamps`,
`ts`). If yours differs, set it with `--time-name`:

```sh
sift-cli import hdf5 ./capture.h5 \
  --asset sensor-array \
  --schema one-d \
  --time-name epoch_ns \
  --time-format absolute-unix-nanoseconds
```

## two-d

Channels are columns of a single 2-D dataset. Identify the time column by index
with `--time-index` (defaults to `0`):

```sh
sift-cli import hdf5 ./capture.h5 \
  --asset sensor-array \
  --schema two-d \
  --time-index 0 \
  --time-format absolute-unix-microseconds
```

## compound

Channels are fields of a compound dataset. Identify the time field by name with
`--time-field`:

```sh
sift-cli import hdf5 ./capture.h5 \
  --asset sensor-array \
  --schema compound \
  --time-field t \
  --time-format absolute-rfc3339
```

## Relative timestamps

For any schema, a relative time format needs an anchor:

```sh
sift-cli import hdf5 ./capture.h5 \
  --asset sensor-array \
  --schema one-d \
  --time-format relative-milliseconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

## Options

| Flag                          | Description                                                              |
| ----------------------------- | ----------------------------------------------------------------------- |
| `--asset`, `-a`               | Asset the data belongs to (required).                                   |
| `--schema`                    | `one-d`, `two-d`, or `compound` (required).                             |
| `--run`, `-r`                 | Run name to associate with the import.                                  |
| `--run-id`                    | Attach to an existing run by ID (takes precedence over `--run`).        |
| `--time-format`               | Time format used by the time dataset/column.                            |
| `--relative-start-time`, `-s` | RFC 3339 start time for relative time formats.                          |
| `--time-index`                | (two-d / compound) Index of the time column/field. Default `0`.         |
| `--time-field`                | (compound) Name of the time field.                                      |
| `--time-name`                 | (one-d) Leaf name of the time dataset when it is not auto-detected.     |
| `--preview`, `-p`             | Print the parsed schema without uploading.                             |
| `--wait`, `-w`                | Block until Sift finishes processing.                                 |

`--time-index`, `--time-field`, and `--time-name` are mutually exclusive; use
the one that matches your schema.
