# httpman

HTTP client CLI for the `lady` workspace — make HTTP requests from the
terminal (an `httpie`-style learning project).

## Features

- `GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `PATCH` subcommands
- `key:value` arguments become request headers
- `key=value` arguments become JSON body fields
- Pretty-prints JSON responses
- Colored status / headers output
- Shell completion generation

## Installation

```bash
cargo install --path httpman
# or from the workspace root
cargo build --release -p httpman
```

## Usage

```bash
httpman get https://httpbin.org/get

httpman post https://httpbin.org/post user=admin X-Token:secret
# -> sends {"user":"admin"} as the JSON body with header X-Token: secret

httpman put https://httpbin.org/put user=admin
httpman delete https://httpbin.org/delete
httpman patch https://httpbin.org/patch user=admin
httpman head https://httpbin.org/get

# Shell completions
httpman completions --shell powershell
```

## Commands

| Command | Description |
| --- | --- |
| `get <URL>` | Send a GET request |
| `post <URL> [PAIRS...]` | Send a POST request with JSON body |
| `put <URL> [PAIRS...]` | Send a PUT request with JSON body |
| `delete <URL>` | Send a DELETE request |
| `head <URL>` | Send a HEAD request |
| `patch <URL> [PAIRS...]` | Send a PATCH request with JSON body |
| `completions --shell <SHELL>` | Generate shell completions |

Pair syntax: `key=value` → JSON body field, `key:value` → request header.
Any `:` in the argument makes it a header (headers may contain further `:`,
params may contain further `=`; splitting happens on the first separator only).

## Exit codes

- `0` — success
- `1` — request or parsing error (message on stderr)

## Development

The crate also contains `macro_rules!` teaching examples (`src/util.rs`,
`tests/test_*.rs`) kept from the original study project.

```bash
cargo test -p httpman
cargo clippy -p httpman --all-targets -- -D warnings
cargo fmt -p httpman -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
