# Verifying Your Setup

Once you have a configured profile, confirm the CLI can reach Sift and that your
credentials are valid:

```sh
sift-cli ping
```

`ping` calls the Sift API and prints the server's response. A successful reply
means your `grpc_uri` and `apikey` are correct and the network path is open.

To verify a specific profile:

```sh
sift-cli --profile mission ping
```

For a non-TLS environment:

```sh
sift-cli --disable-tls ping
```

## Troubleshooting

| Symptom                                   | Likely cause and fix                                                                 |
| ----------------------------------------- | ------------------------------------------------------------------------------------ |
| `expected to find 'sift.toml'`            | No config file yet. Run `sift-cli config create`, then `config update`.              |
| `Expected value of 'apikey' to be present`| The profile is missing a field. Re-run `sift-cli config update` with the missing value. |
| `Profile '<name>' not found`              | The named profile does not exist. Check `sift-cli config show`.                      |
| Authentication or permission errors       | The API key is wrong or lacks access. Generate a new key in the Sift web app.        |
| Connection or TLS errors                  | Check the URIs. For non-TLS environments add `--disable-tls`.                        |

Once `ping` succeeds you are ready to [import data](../data/importing.md).
