# Managing the Sift agent integration

Use the CLI lifecycle instead of editing one client's MCP configuration or
skill:

- Run `sift-cli agent doctor` to diagnose setup. It checks the installed
  profile, access mode, and profile `app_uri` without a change.
- If doctor finds a missing `app_uri`, follow its guidance. For a production
  or government profile, propose `sift-cli agent doctor --fix` for the default
  profile. For a named profile, add `--profile <name>` before `agent`. Show the
  profile and exact command, then wait for approval. This command sets only a
  known public Sift app URL. Ask the user to restart the MCP client after the
  fix.
- For any other domain, do not guess `app_uri`. Ask the user to open their Sift
  web app. Ask them to copy the URL origin from the browser address bar. The
  origin contains the scheme and host. It can use any top-level domain. Then
  propose `sift-cli --profile <name> config update --app-uri
  <SIFT_WEB_ORIGIN>`. Omit `--profile` for the default profile. Wait for
  approval before the config change. Ask for an MCP client restart afterward.
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
