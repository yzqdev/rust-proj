# taoqi

JSON processor CLI for the `lady` workspace: format, validate, minify and
query JSON files.

## Features

- `format` — pretty-print a JSON file (optionally in-place)
- `validate` — check a file is valid JSON and report its root type
- `minify` — remove whitespace (optionally in-place, reports byte savings)
- `query` — navigate values by dotted key path, numeric segments index arrays
- Clear error messages with a non-zero exit code on failure
- Shell completion generation

## Installation

```bash
cargo install --path taoqi
# or from the workspace root
cargo build --release -p taoqi
```

## Usage

```bash
taoqi format data.json            # print pretty JSON
taoqi format data.json --in-place

taoqi validate data.json          # ✅ `data.json` is valid JSON (root type: object)

taoqi minify data.json            # {"a":1}
taoqi minify data.json --in-place # Minified `data.json` in-place (24 bytes -> 7 bytes)

taoqi query data.json name
taoqi query data.json address.city
taoqi query data.json items.0     # array index

# Shell completions
taoqi completions --shell bash
```

## Commands

| Command | Description |
| --- | --- |
| `format <FILE> [--in-place]` | Pretty-print / format a JSON file |
| `validate <FILE>` | Validate a JSON file |
| `minify <FILE> [--in-place]` | Minify a JSON file |
| `query <FILE> <PATH>` | Query a JSON value by key path |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (unreadable file, invalid JSON, missing key, bad index)

## Development

```bash
cargo test -p taoqi
cargo clippy -p taoqi --all-targets -- -D warnings
cargo fmt -p taoqi -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
