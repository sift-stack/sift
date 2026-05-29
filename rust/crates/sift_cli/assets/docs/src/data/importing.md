# Importing Data

`sift-cli import` loads time-series files into Sift. Every import attaches data
to an **asset** (the thing producing the data) and, optionally, a **run** (a
named window of activity, such as a single test).

## Supported file types

| Type    | Command                          | Chapter                          |
| ------- | -------------------------------- | -------------------------------- |
| CSV     | `import csv`                     | [CSV](./import-csv.md)           |
| Parquet | `import parquet flat-dataset`    | [Parquet](./import-parquet.md)   |
| TDMS    | `import tdms`                    | [TDMS](./import-tdms.md)         |
| HDF5    | `import hdf5`                    | [HDF5](./import-hdf5.md)         |
| Backups | `import backups`                | [Backups](./import-backups.md)   |

If your data is in a format not listed here, convert it to CSV or Parquet first
and import that, or stream it in with the
[Sift Python or Rust libraries](#when-to-stream-instead).

## Concepts common to every import

These flags and behaviors apply across import commands.

### Asset and run

- `--asset <name>` (required): the asset the data belongs to. Sift creates the
  asset if it does not exist.
- `--run <name>` (optional): associate the data with a named run.
- `--run-id <id>` (optional, where supported): attach to an existing run by ID.
  Takes precedence over `--run`.

### Preview before uploading

Add `--preview` (`-p`) to parse the file and print the schema the CLI inferred
without sending anything to Sift. Use this to confirm channel names, data types,
and time parsing before committing to an upload.

```sh
sift-cli import csv ./telemetry.csv --asset rover-1 --preview
```

### Wait for processing

Imports are processed asynchronously by Sift. By default the CLI returns once
the data is uploaded. Add `--wait` (`-w`) to block until Sift finishes
processing the import, which is useful in scripts that depend on the data being
queryable immediately afterward.

```sh
sift-cli import csv ./telemetry.csv --asset rover-1 --wait
```

### Time formats

Imports that parse a time column accept a `--time-format` describing how
timestamps are encoded. Supported values:

| Value                        | Meaning                                  |
| ---------------------------- | ---------------------------------------- |
| `absolute-rfc3339` (default) | RFC 3339 timestamps, e.g. `2026-05-29T12:00:00Z` |
| `absolute-datetime`          | Datetime strings                         |
| `absolute-unix-seconds`      | Unix epoch seconds                       |
| `absolute-unix-milliseconds` | Unix epoch milliseconds                  |
| `absolute-unix-microseconds` | Unix epoch microseconds                  |
| `absolute-unix-nanoseconds`  | Unix epoch nanoseconds                   |
| `relative-nanoseconds`       | Offset from a start time, in nanoseconds |
| `relative-microseconds`      | Offset from a start time, in microseconds|
| `relative-milliseconds`      | Offset from a start time, in milliseconds|
| `relative-seconds`           | Offset from a start time, in seconds     |
| `relative-minutes`           | Offset from a start time, in minutes     |
| `relative-hours`             | Offset from a start time, in hours       |

When the time format is **relative**, supply a start time with
`--relative-start-time` (`-s`) in RFC 3339 so Sift can anchor the offsets to
absolute time.

### Channel data types

Where a command lets you declare channel types, the accepted values are:
`infer`, `double`, `float`, `int32`, `uint32`, `int64`, `uint64`, `bool`,
`string`, `enum`, `bit-field`, and `bytes`. Use `infer` to let the CLI choose
the type while you still set the unit or description.

## When to stream instead

File import is the right tool for data at rest. For live or very large data,
stream directly into Sift with the client libraries:

- **Python:** [ingestion examples](https://sift-stack.github.io/sift/python/latest/examples/ingestion/)
- **Rust:** [`sift_stream`](https://docs.rs/sift_stream/latest/sift_stream/)
