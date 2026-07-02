# Importing ULog

```
sift-cli import ulog <PATH> --asset <NAME> [OPTIONS]
```

ULog (`.ulg`) is the PX4 autopilot logging format. The CLI imports every
logged topic in the file, one channel per field, named
`<message>_<multi_id>.<field>` (for example `sensor_accel_0.x`). Logged status
text becomes `log_messages` channels.

## Quick start

```sh
sift-cli import ulog ./flight.ulg --asset quadcopter-1
```

Attach to a run and wait for processing:

```sh
sift-cli import ulog ./flight.ulg \
  --asset quadcopter-1 \
  --run "test flight 2026-07-02" \
  --wait
```

## Anchoring timestamps

ULog records boot-relative timestamps. The import anchors them to UTC using
the log's GPS time fix. If the log has no GPS fix, or you want to override it,
pass the UTC time of log start:

```sh
sift-cli import ulog ./flight.ulg \
  --asset quadcopter-1 \
  --relative-start-time 2026-07-02T15:04:05Z
```

## Importing log metadata

ULog files carry info messages (like `sys_name` and `ver_hw`) and initial
parameter values. Import selected keys as run metadata with `--info-key` and
`--param-key`; both require `--run` or `--run-id`:

```sh
sift-cli import ulog ./flight.ulg \
  --asset quadcopter-1 \
  --run "test flight 2026-07-02" \
  --info-key sys_name \
  --info-key ver_hw \
  --param-key MC_PITCH_P
```

Info keys are stored as `info.<key>` and parameters as `param.<name>`. The
import fails if a requested key is not present in the file.

## Recoverable parse errors

A log may end with a truncated record or contain corrupt segments (for
example, after a power loss). By default the import fails on such errors; pass
`--parse-error-policy ignore-error` to import what parsed cleanly instead:

```sh
sift-cli import ulog ./flight.ulg \
  --asset quadcopter-1 \
  --parse-error-policy ignore-error
```

## Options

| Flag                          | Description                                                          |
| ----------------------------- | -------------------------------------------------------------------- |
| `--asset`, `-a`               | Asset the data belongs to (required).                                |
| `--run`, `-r`                 | Run name to associate with the import.                               |
| `--run-id`                    | Attach to an existing run by ID (takes precedence over `--run`).     |
| `--relative-start-time`, `-s` | UTC time of log start; overrides the log's GPS time fix.             |
| `--info-key`                  | Info key to import as run metadata; repeatable.                      |
| `--param-key`                 | Parameter name to import as run metadata; repeatable.                |
| `--parse-error-policy`        | Handling for recoverable parse errors (default `fail-on-error`).     |
| `--preview`, `-p`             | Print the parsed schema without uploading.                           |
| `--wait`, `-w`                | Block until Sift finishes processing.                                |
