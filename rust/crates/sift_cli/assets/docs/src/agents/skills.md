# Agent Integration

The Sift agent integration is one release-matched bundle: a Sift skill plus the
MCP sidecar already embedded in `sift-cli`. One command detects supported AI
coding clients and configures all of them together:

```sh
sift-cli agent install
```

> **The Sift MCP server and `agent` commands are in active development.** They
> remain behind the `mcp` Cargo feature in 0.3.0 and are intended for local
> development and evaluation before the open beta.

## Supported clients

| Client      | Skill location                            | MCP setup                                      |
| ----------- | ----------------------------------------- | ---------------------------------------------- |
| Claude Code | `~/.claude/skills/sift/SKILL.md`          | User-scoped Claude MCP registration            |
| Codex       | `~/.agents/skills/sift/SKILL.md`          | User-scoped Codex MCP registration             |
| Cursor      | `~/.agents/skills/sift/SKILL.md`          | `sift` entry in `~/.cursor/mcp.json`            |
| OpenCode    | `~/.agents/skills/sift/SKILL.md`          | `sift` entry in the user `opencode.json`        |

Codex, Cursor, and OpenCode intentionally share one Agent Skills standard
installation. There are no generated `AGENTS.md` copies and no per-client skill
versions.

## Lifecycle commands

```sh
sift-cli agent install
sift-cli agent update
sift-cli agent doctor
sift-cli agent uninstall
```

- `install` writes the current CLI's embedded bundle to every detected client.
  MCP registrations are read-only by default; pass `--allow-destructive` to
  opt every detected MCP client in during initial setup. It uses the default
  Sift profile unless `--profile <name>` selects a named profile.
- `update` first checks for a newer stable `sift-cli` release. If the CLI is
  outdated, it prints the exact version-pinned curl or PowerShell installer and
  asks you to rerun the command. Otherwise it refreshes every detected client
  and preserves the current access mode and profile.
- `doctor` checks the CLI release, installed skill contents, and MCP
  registrations, including their profile and read-only or destructive access
  mode. It prints the same version-pinned installer when the CLI is outdated.
- `uninstall` removes Sift-managed skill files and MCP registrations from every
  detected client.

Switch access for all detected MCP clients in lockstep:

```sh
sift-cli agent update --allow-destructive
sift-cli agent update --read-only
```

The first command requires an intentional user choice because it exposes tools
that modify or archive existing Sift resources. Reload or restart the clients
after changing access. If clients have mixed modes, ordinary `update` and
`doctor` report the inconsistency and ask you to choose one of these commands
instead of silently choosing a mode.

Switch profiles for all detected MCP clients in lockstep:

```sh
sift-cli agent update --profile mission
sift-cli agent update --default-profile
```

The first command selects a named profile. The second removes `--profile` from
every registration so the CLI's default profile is used. An ordinary update
preserves the uniformly installed profile. If clients have mixed profiles,
`update` and `doctor` require one of these explicit choices.

These commands do not maintain a Sift state file. Status comes from the actual
skill contents and deterministic MCP config keys. Install and update preflight
all detected targets before writing; if a custom skill or custom MCP server
already uses the name `sift`, the command leaves everything untouched and
reports the conflict. Uninstall likewise leaves unmanaged content in place.
Uninstall performs the same all-client preflight, so a conflict prevents a
partial removal.

OpenCode configurations written as `opencode.jsonc` are diagnosed but not
rewritten because doing so could destroy user comments. Convert that file to
plain `opencode.json` or add the reported Sift entry manually.

If you previously ran `install agent-skills agents-md`, remove that legacy Sift
block from the affected project's `AGENTS.md` manually. The new lifecycle is
intentionally user-scoped and cannot safely discover or edit old project files.

## What the skill covers

The single installed skill documents the recommended Sift toolbox:

1. The Sift MCP server.
2. `sift-cli`.
3. The Sift REST API over cURL.
4. The Sift Python library (`sift_client`) and Rust streaming library
   (`sift_stream`).

It also explains when to import files, stream data, query locally, or open data
in Sift Explore.
