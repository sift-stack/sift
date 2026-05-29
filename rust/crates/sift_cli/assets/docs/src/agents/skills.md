# Agent Skills

`sift-cli install agent-skills` installs a Sift "skill", a short instruction file
that teaches an AI coding assistant how to work with Sift: which tool to reach
for, the order to prefer them in, and how to import or stream data.

```
sift-cli install agent-skills <AGENT> [--output <PATH>] [--print]
```

## Supported agents

| Agent          | File        | Default location                                  |
| -------------- | ----------- | ------------------------------------------------- |
| `claude-code`  | `SKILL.md`  | `~/.claude/skills/sift/SKILL.md`                  |
| `agents-md`    | `AGENTS.md` | `AGENTS.md` at the root of your current project   |

`agents-md` follows the open [AGENTS.md](https://agents.md) standard recognized
by Codex, Cursor, Aider, Zed, Windsurf, Amp, Jules, Factory, and RooCode.

## Installing

For Claude Code (writes to your per-user skills directory):

```sh
sift-cli install agent-skills claude-code
```

For the AGENTS.md ecosystem (writes to the current project root):

```sh
sift-cli install agent-skills agents-md
```

## Choosing where it goes

Write the skill to a specific path with `--output`:

```sh
sift-cli install agent-skills agents-md --output ./docs/AGENTS.md
```

Or print it to stdout without writing anything, to review or pipe it elsewhere:

```sh
sift-cli install agent-skills claude-code --print
```

## What the skill covers

The installed file documents your Sift toolbox and the recommended order of
preference for agents:

1. The Sift MCP server (see [MCP Server](./mcp.md)).
2. `sift-cli` itself.
3. The Sift REST API over cURL.
4. The Sift Python library (`sift_client`) and Rust streaming library
   (`sift_stream`).

It also explains how to import files versus stream data, so your assistant picks
the right path for a given task.
