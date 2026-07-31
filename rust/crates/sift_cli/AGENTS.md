# sift_cli — agent skill guidance

## One canonical skill

`assets/skills/sift/SKILL.md` is the only source for Sift's agent instructions.

Do not add per-client copies of the skill. The pair of near-identical files this
replaced drifted apart, which is the failure this layout exists to prevent.

## Updating the skill

Keep the skill accurate to the CLI and `sift_mcp` tool surfaces. In particular:

- The MCP tool list must mirror the tools registered by `sift_mcp`.
- The `sift-cli` subcommand list must mirror `src/cli/mod.rs`.
- Keep the preference order MCP → CLI → REST/cURL → Python.
- Keep `sift_client` as the recommended Python module; `sift_py` is deprecated.

Write in direct voice and keep it concise. The skill is loaded under context
pressure, so every line should change what the agent does.
