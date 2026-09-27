# phoebe

CSV processor & package installer CLI for the `lady` workspace.

## Features

- Convert CSV files to JSON (compact or pretty)
- Configurable field delimiter, optional header row
- Malformed rows are skipped with a warning on stderr
- Package install simulation (`install` subcommand)
- Shell completion generation

## Installation

```bash
cargo install --path phoebe
# or from the workspace root
cargo build --release -p phoebe
```

## Usage

```bash
# Convert a CSV file (defaults: --input input.csv --output output.json)
phoebe csv --input data.csv --output data.json

# Custom delimiter, no header row, pretty output
phoebe csv --input data.csv --output data.json --delimiter ';' --header=false --pretty

# Package installer simulation
phoebe install left-pad
phoebe install left-pad --latest --global

# Shell completions
phoebe completions --shell bash
```

## Commands

| Command | Description |
| --- | --- |
| `csv` | Convert CSV to JSON (`-i/--input`, `-o/--output`, `-d/--delimiter`, `--header`, `-p/--pretty`) |
| `install [NAME]` | Simulate installing a package (`--latest`, `--param`, `-g/--global`) |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (unreadable input, unwritable output, invalid delimiter)

## Development

```bash
cargo test -p phoebe
cargo clippy -p phoebe --all-targets -- -D warnings
cargo fmt -p phoebe -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
