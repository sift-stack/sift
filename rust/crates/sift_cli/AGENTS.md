# sift_cli — agent skill guidance

## One canonical skill

`assets/skills/sift/SKILL.md` is the only source for Sift's agent instructions.

Do not add per-client copies of the skill. The pair of near-identical files this
replaced drifted apart, which is the failure this layout exists to prevent.
Claude installs the canonical file to `~/.claude/skills/sift/SKILL.md`; Codex,
Cursor, and OpenCode share `~/.agents/skills/sift/SKILL.md`.

The skill is embedded at compile time, so rebuild `sift-cli` after changing it.

## CLI lifecycle

`src/cmd/agent/` implements four stateless, user-scoped commands:

- `sift-cli agent install`
- `sift-cli agent update`
- `sift-cli agent doctor`
- `sift-cli agent uninstall`

Install and update operate on every detected supported client. They preflight
all targets before writing and refuse to overwrite an unmanaged same-name skill
or MCP entry. Doctor derives status from the installed files and client configs.
Uninstall removes only content identifiable as Sift-managed. Do not add a state
file or per-client version tracking.

Install defaults every MCP registration to read-only. Install accepts
`--allow-destructive` as an explicit opt-in. Install uses the default Sift
profile unless `--profile <name>` selects a named profile. An ordinary update
preserves the single access mode and profile discovered from the managed client
configs. Update accepts `--allow-destructive` or `--read-only` to switch access,
`--profile <name>` to switch to a named profile, and `--default-profile` to
switch back to the default; each change applies to every detected client.
Doctor reports both settings. Mixed modes or profiles are errors rather than
values the CLI silently resolves.

The `agent` command and `mcp` sidecar remain behind the `mcp` Cargo feature until
the open-beta release explicitly changes that policy.

## Updating the skill

Keep the skill accurate to the CLI and `sift_mcp` tool surfaces. In particular:

- The MCP tool list must mirror the tools registered by `sift_mcp`.
- The `sift-cli` subcommand list must mirror `src/cli/mod.rs`.
- Keep the preference order MCP → CLI → REST/cURL → Python.
- Keep `sift_client` as the recommended Python module; `sift_py` is deprecated.
- Teach agents to use `agent doctor`, `install`, and `update` instead of editing
  one client. They must obtain explicit user approval before running
  `agent update --allow-destructive` and tell the user to reload the client.

Write in direct voice and keep it concise. The skill is loaded under context
pressure, so every line should change what the agent does.

## Local development

From the repository root:

```sh
cargo build -p sift_cli --features mcp
cargo test -p sift_cli --features mcp cmd::agent
./target/debug/sift-cli agent doctor
```

`agent install` changes real user-level client configuration and points it at
the exact `sift-cli` executable running the command. Use `doctor` for read-only
validation, and only run install/update/uninstall when those user-level changes
are intended.
