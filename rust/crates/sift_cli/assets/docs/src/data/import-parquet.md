# Importing Parquet

```
sift-cli import parquet flat-dataset <PATH> --asset <NAME> --time-path <PATH> --time-format <FORMAT> [OPTIONS]
```

The `flat-dataset` layout expects a Parquet file where every column maps to a
single channel, except for one time column. This is the common shape produced by
data loggers and by exporting from a dataframe.

> Parquet columns are addressed by **path**, not by index. For a flat file the
> path is just the column name. For nested columns it is the dotted path to the
> leaf.

## Quick start

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --time-path timestamp \
  --time-format absolute-unix-nanoseconds
```

Preview first, and attach to a run:

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --run "2026-05-29 field test" \
  --time-path timestamp \
  --time-format absolute-unix-nanoseconds \
  --preview
```

## Selecting and typing channels

By default all non-time columns are imported. To restrict the set or override
types and metadata, name each column with `--channel-path` and supply matching
`--data-type`, `--unit`, and `--description` in the same order:

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --time-path timestamp \
  --time-format absolute-unix-nanoseconds \
  --channel-path speed --data-type double --unit "m/s" \
  --channel-path gear  --data-type enum --enum-config "0,park|1,drive"
```

`--bit-field-config` uses `<index,name,bit_count>` triplets for this command.

## Relative timestamps

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --time-path elapsed_ms \
  --time-format relative-milliseconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

## Complex column types

If the file contains maps, lists, or structs, control how they are handled with
`--complex-types-mode` (`-m`). Run the command with `--help` to see the
supported strategies for your version.

## Options

| Flag                          | Description                                                  |
| ----------------------------- | ------------------------------------------------------------ |
| `--asset`, `-a`               | Asset the data belongs to (required).                        |
| `--time-path`, `-t`           | Path to the time column (required).                          |
| `--time-format`, `-f`         | Time format (required).                                      |
| `--run`, `-r`                 | Run name to associate with the import.                       |
| `--relative-start-time`, `-s` | RFC 3339 start time for relative time formats.               |
| `--channel-path`, `-c`        | Column path to import; repeatable.                           |
| `--data-type`, `-d`           | Data type per `--channel-path`; repeatable.                  |
| `--unit`, `-u`                | Unit per `--channel-path`; repeatable.                       |
| `--description`, `-n`         | Description per `--channel-path`; repeatable.                |
| `--enum-config`, `-e`         | Enum `<key,name>` pairs for enum channels.                   |
| `--bit-field-config`, `-b`    | Bit-field `<index,name,bit_count>` triplets.                 |
| `--complex-types-mode`, `-m`  | Strategy for maps, lists, and structs.                       |
| `--preview`, `-p`             | Print the parsed schema without uploading.                   |
| `--wait`, `-w`                | Block until Sift finishes processing.                        |
