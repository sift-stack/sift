# sift_cli — guidance for Claude

## Agent skill files

`sift-cli` ships two agent-facing instruction files and installs them with
`sift-cli install agent-skills <agent>`:

- `assets/skills/claude-code/SKILL.md` — Claude Code skill. Has YAML
  frontmatter (`name`, `description`) and installs to the user's Claude skills
  directory.
- `assets/skills/agents-md/AGENTS.md` — the open AGENTS.md standard (Codex,
  Cursor, Aider, Zed, Windsurf, Amp, Jules, Factory, RooCode). No frontmatter;
  installs at the project root.

Both are embedded into the binary at build time via `include_str!` in
`src/cmd/install/agent.rs`. Editing the files changes what the CLI installs, so
rebuild after any change.

## Keeping the two files in lockstep

The two files exist because Claude Code and the AGENTS.md ecosystem expect
different on-disk formats. Their **content is the same** and must not drift.

The contract:

1. **The body is shared.** Everything from the `# Sift toolbox` heading to the
   end of file must be byte-for-byte identical between `SKILL.md` and
   `AGENTS.md`.
2. **Only the headers differ.** `SKILL.md` carries YAML frontmatter above the
   body. `AGENTS.md` has no frontmatter. Both carry the lockstep HTML comment
   at the top.
3. **Edit both in the same change.** When you add, remove, or reword anything
   in the body, apply the identical edit to the other file in the same commit.
   Never update one alone.
4. **The frontmatter `description` tracks the body.** When you change which
   tasks or tools the skill covers, update the `description` in `SKILL.md` so
   its trigger phrases still match the body.

Verify the bodies match before committing:

```sh
cd rust/crates/sift_cli/assets/skills
diff <(awk '/^# Sift toolbox/{f=1} f' claude-code/SKILL.md) \
     <(awk '/^# Sift toolbox/{f=1} f' agents-md/AGENTS.md)
```

No output means they are in sync. Any diff must be reconciled before the change
is done.

## Updating the skill content

Keep the body accurate to the actual tooling. When you change the CLI's
commands, the MCP server's tools, or the recommended workflow, update the skill
body to match. In particular:

- The MCP tool list (`list_assets`, `list_runs`, `list_channels`, `get_data`,
  `sql`, `upload_dataset`) must mirror the tools in the `sift_mcp` crate. If a
  tool is added or removed there, update both skill files here.
- The `sift-cli` subcommand list must mirror `src/cli/mod.rs`. If you add an
  import format or a new subcommand, reflect it in the body.
- Keep the tool preference order (MCP → `sift-cli` → REST/cURL → Python)
  intact unless the product guidance itself changes.
- Keep `sift_client` as the recommended Python module; `sift_py` is deprecated
  and stays a last resort.

Write in direct voice and keep it concise. The body is read by other agents
under context pressure, so every line should change what the agent does.
