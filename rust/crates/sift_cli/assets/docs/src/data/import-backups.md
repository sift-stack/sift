# Importing Backups

```
sift-cli import backups [OPTIONS]
sift-cli import backups ls [PATH]
```

When `sift_stream` streams data to Sift, it can write local backup files so no
data is lost if the connection drops. The `import backups` command replays those
files into Sift once connectivity is restored.

## Listing backups

Show the backup files the CLI can see. With no path, it looks in your OS data
directory:

```sh
sift-cli import backups ls
sift-cli import backups ls /path/to/backups
```

## Replaying backups

Import every backup file in a directory:

```sh
sift-cli import backups --path /path/to/backups
```

With no `--path`, the default OS data directory is used:

```sh
sift-cli import backups
```

## Cleaning up after a successful import

Add `--cleanup` to delete each backup file after it uploads successfully:

```sh
sift-cli import backups --path /path/to/backups --cleanup
```

## Options

| Flag              | Description                                                              |
| ----------------- | ----------------------------------------------------------------------- |
| `--path`, `-p`    | Directory containing backup files (defaults to the OS data directory).  |
| `--cleanup`, `-c` | Delete backup files after a successful upload.                          |
