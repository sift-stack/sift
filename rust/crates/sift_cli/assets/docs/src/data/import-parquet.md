# Importing Parquet

Parquet imports come in two shapes:

- `flat-dataset` — every column is a channel, with one time column (the common
  shape produced by data loggers and by exporting from a dataframe).
- `cpr` (channel-per-row) — one channel per row, either a single channel for
  the whole file or a name column identifying the channel per row.

## Flat dataset

```
sift-cli import parquet flat-dataset <PATH> --asset <NAME> [OPTIONS]
```

The `flat-dataset` layout expects a Parquet file where every column maps to a
single channel, except for one time column.

> Parquet columns are addressed by **path**, not by index. For a flat file the
> path is just the column name. For nested columns it is the dotted path to
> the leaf.

### Quick start

The CLI auto-detects the time column from common names (`time`, `timestamp`,
`timestamps`, `ts`) and infers the time format from the column's Arrow type:

```sh
sift-cli import parquet flat-dataset ./run.parquet --asset rover-1
```

If detection needs help:

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --time-path timestamp \
  --time-format absolute-unix-nanoseconds
```

### Selecting and typing channels

By default all non-time columns are imported. To restrict the set or override
types and metadata, name each column with `--channel-path` and supply matching
`--data-type`, `--unit`, and `--description` in the same order:

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --channel-path speed --data-type double --unit "m/s" \
  --channel-path gear  --data-type enum --enum-config "0,park|1,drive"
```

`--bit-field-config` uses `<name,index,length>` triplets (e.g.
`"12v,0,4|led,4,4"`).

### Relative timestamps

```sh
sift-cli import parquet flat-dataset ./run.parquet \
  --asset rover-1 \
  --time-path elapsed_ms \
  --time-format relative-milliseconds \
  --relative-start-time 2026-05-29T00:00:00Z
```

### Complex column types

If the file contains maps, lists, or structs, control how they are handled
with `--complex-types-mode` (`-m`). Run the command with `--help` to see the
supported strategies for your version.

### Options

| Flag                          | Description                                                                                       |
| ----------------------------- | ------------------------------------------------------------------------------------------------- |
| `--asset`, `-a`               | Asset the data belongs to (required).                                                             |
| `--time-path`, `-t`           | Path to the time column. Auto-detected from `time`, `timestamp`, `timestamps`, `ts` if omitted.   |
| `--time-format`, `-f`         | Time format. Inferred from the time column's Arrow type if omitted.                               |
| `--run`, `-r`                 | Run name to associate with the import.                                                            |
| `--relative-start-time`, `-s` | RFC 3339 start time for relative time formats.                                                    |
| `--channel-path`, `-c`        | Column path to import; repeatable.                                                                |
| `--data-type`, `-d`           | Data type per `--channel-path`; repeatable. Use `infer` to derive from the Parquet/Arrow schema.  |
| `--unit`, `-u`                | Unit per `--channel-path`; repeatable.                                                            |
| `--description`, `-n`         | Description per `--channel-path`; repeatable.                                                     |
| `--enum-config`, `-e`         | Enum `<key,name>` pairs for enum channels.                                                        |
| `--bit-field-config`, `-b`    | Bit-field `<name,index,length>` triplets.                                                         |
| `--complex-types-mode`, `-m`  | Strategy for maps, lists, and structs.                                                            |
| `--preview`, `-p`             | Print the parsed schema without uploading.                                                        |
| `--wait`, `-w`                | Block until Sift finishes processing.                                                             |

## Channel-per-row (cpr)

Use `cpr` when the Parquet file has one channel per row instead of one channel
per column. There are two modes:

- `single` — every row belongs to the same channel.
- `multi` — a name column identifies which channel each row belongs to.

### Single mode

```
sift-cli import parquet cpr single <PATH> --asset <NAME> --data-path <COL> --channel-name <NAME> [OPTIONS]
```

```sh
sift-cli import parquet cpr single ./readings.parquet \
  --asset rover-1 \
  --data-path value \
  --channel-name temperature
```

### Multi mode

```
sift-cli import parquet cpr multi <PATH> --asset <NAME> --data-path <COL> --name-path <COL> [OPTIONS]
```

```sh
sift-cli import parquet cpr multi ./readings.parquet \
  --asset rover-1 \
  --data-path value \
  --name-path channel
```

### Options

| Flag                          | Description                                                                                       |
| ----------------------------- | ------------------------------------------------------------------------------------------------- |
| `--asset`, `-a`               | Asset the data belongs to (required).                                                             |
| `--data-path`                 | Path to the column holding values (required).                                                     |
| `--channel-name`              | (`single`) Channel name for every row (required).                                                 |
| `--name-path`                 | (`multi`) Path to the column holding channel names (required).                                    |
| `--time-path`, `-t`           | Path to the time column. Auto-detected from `time`, `timestamp`, `timestamps`, `ts` if omitted.   |
| `--time-format`, `-f`         | Time format. Inferred from the time column's Arrow type if omitted.                               |
| `--run`, `-r`                 | Run name to associate with the import.                                                            |
| `--relative-start-time`, `-s` | RFC 3339 start time for relative time formats.                                                    |
| `--data-type`                 | (`single`) Data type. Use `infer` to derive from the Parquet/Arrow schema.                        |
| `--unit`                      | (`single`) Channel units.                                                                          |
| `--description`, `-n`         | (`single`) Channel description.                                                                    |
| `--complex-types-mode`, `-m`  | Strategy for maps, lists, and structs.                                                            |
| `--preview`, `-p`             | Print the parsed schema without uploading.                                                        |
| `--wait`, `-w`                | Block until Sift finishes processing.                                                             |
