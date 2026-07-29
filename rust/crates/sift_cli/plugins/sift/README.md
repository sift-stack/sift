# Sift agent plugin

This directory is the canonical, release-coupled Sift agent plugin source. It
packages one Sift skill and descriptors that launch the `sift-cli mcp` sidecar
for Codex, Claude Code, Cursor, OpenCode, and Pi conventions.

The skill is embedded into the CLI binary, while this complete plugin directory
is included in the `sift_cli` Cargo package. It is not versioned or updated
independently. Its manifest versions must match
`rust/crates/sift_cli/Cargo.toml`, and `sift-cli agent install` / `update`
deploy the release-matched bundle to every detected client together.
Install defaults to read-only MCP registrations. Use
`agent install --allow-destructive` to opt in initially; ordinary updates
preserve the current mode, while `agent update --allow-destructive` and
`agent update --read-only` switch every detected client together.

Pi consumes the shared Agent Skills directory and does not receive an MCP
registration because Pi has no built-in MCP client. OpenCode consumes that same
skill plus a user-level local MCP entry.
