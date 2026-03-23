# Getting Started

## Table of Contents

- [Installation](#installation)
  - [Unix](#unix)
  - [Windows](#windows)
  - [Bootstrapping the CLI config](#bootstrapping-the-cli-config)
  - [Shell Autocompletion](#shell-autocompletion)

## Installation

### Unix

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/sift-stack/sift/releases/download/sift_cli-v0.1.0/sift_cli-installer.sh | sh
```

### Windows

```bash
powershell -ExecutionPolicy Bypass -c "irm https://github.com/sift-stack/sift/releases/download/sift_cli-v0.1.0/sift_cli-installer.ps1 | iex"
```

### Bootstrapping the CLI config

Start by creating a config file which will live your user's [data directory](https://docs.rs/dirs/latest/dirs/fn.data_dir.html).

```bash
sift-cli config create
```

Create your default profile:

```bash
sift-cli config update \
  --grpc-uri $SIFT_GRPC_URI \
  --rest-uri $SIFT_REST_URI \
  --api-key $MY_API_KEY 
```

Now you will be able to execute commands without needing to explicitly specify a `--profile`.

### Shell Autocompletion

To install shell autocompletions run the following:

```bash
sift-cli completions update
```

Restart your shell and try tab-autocompleting.
