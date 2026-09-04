# Guides

Conceptual references for the Sift Python client. Guides explain how a feature
works and how to configure it. For runnable, end-to-end walkthroughs see the
[Examples](../examples/index.md) section.

## Available guides

- [Credentials & Profiles](credentials.md): how `SiftClient` resolves its API key
  and endpoints from arguments, environment variables, and the `sift.toml`
  profiles that `sift-cli` manages.
- [Pytest Plugin](pytest_plugin/index.md): turn a pytest run into a `TestReport`
  in Sift. Each test becomes a `TestStep`, measurements are recorded as rows, and
  failures propagate up through nested substeps to the report.
