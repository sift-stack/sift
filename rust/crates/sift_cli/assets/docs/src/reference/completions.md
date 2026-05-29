# Shell Completions

`sift-cli install completions` sets up tab completion for your shell.

## Automatic update

Let the CLI detect your shell and update its completions file:

```sh
sift-cli install completions update
```

## Printing completions

Print a completion script to stdout so you can place it yourself. The CLI infers
your shell from `$SHELL`, or you can name it explicitly:

```sh
sift-cli install completions print
sift-cli install completions print --shell zsh
```

Supported shells include `bash`, `zsh`, `fish`, `powershell`, and `elvish`.

## Manual setup

Source the output from your shell's startup file. For example, with Zsh:

```sh
sift-cli install completions print --shell zsh > ~/.sift-cli-completions.zsh
echo 'source ~/.sift-cli-completions.zsh' >> ~/.zshrc
```

Restart your shell or re-source the startup file for completions to take effect.
