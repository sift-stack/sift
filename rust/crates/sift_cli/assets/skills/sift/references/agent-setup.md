# Managing the Sift agent integration

Use the CLI lifecycle instead of editing one client's MCP configuration or
skill:

- Run `sift-cli agent doctor` to diagnose setup, including the installed
  profile and access mode, without changing it.
- For first-time setup, explain that `sift-cli agent install` installs the
  release-matched skill and read-only MCP registration for every detected
  client using the default Sift profile. If the user selected a named profile,
  pass it as `sift-cli agent install --profile <name>`. Run the command when the
  user asks you to install or approves the change.
- Run `sift-cli agent update` to refresh every detected client together. It
  preserves the existing profile and read-only or destructive access mode.
  Switch every client to another named profile with
  `sift-cli agent update --profile <name>`, or return them to the default with
  `sift-cli agent update --default-profile`.
- If the CLI is outdated, relay the exact curl or PowerShell installer printed
  by `agent doctor` or `agent update`. After the user updates `sift-cli`, rerun
  `sift-cli agent update`.
- Never repair or update only one detected client. If doctor reports mixed
  access modes, ask the user to choose `sift-cli agent update --read-only` or
  `sift-cli agent update --allow-destructive`. If it reports mixed profiles,
  ask for the intended profile and use `sift-cli agent update --profile <name>`
  or `sift-cli agent update --default-profile`.

## Enabling destructive tools

Destructive tools (`update_*`, `archive_*`, `unarchive_*`) are gated on
`--allow-destructive` and disabled by default. When a call is blocked:

1. Explain to the user that this access is disabled by default.
2. Ask for explicit approval to enable it across every detected client.
3. Only after approval, run `sift-cli agent update --allow-destructive`.
4. Ask the user to reload or restart the MCP client.
5. Wait for the user to confirm the restart before you retry the call.

Never enable destructive access silently. Restore safe mode with
`sift-cli agent update --read-only`.
