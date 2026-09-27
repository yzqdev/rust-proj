# asta

Hash calculator CLI for the `lady` workspace. Computes MD5 and SHA256 digests
of strings and files.

## Features

- MD5 / SHA256 of a string
- MD5 / SHA256 of a file (streamed in 8 KiB chunks, works on large files)
- Shell completion generation
- Clear error messages with a non-zero exit code on failure

## Installation

```bash
cargo install --path asta
# or from the workspace root
cargo build --release -p asta
```

## Usage

```bash
# Hash a string
asta md5 "hello"          # MD5("hello") = 5d41402abc4b2a76b9719d911017c592
asta sha256 "hello"

# Hash a file (default algorithm: md5)
asta file data.bin
asta file --algorithm sha256 data.bin

# Shell completions
asta completions --shell bash >> ~/.bashrc
```

## Commands

| Command | Description |
| --- | --- |
| `md5 <TEXT>` | Compute the MD5 hash of a string |
| `sha256 <TEXT>` | Compute the SHA256 hash of a string |
| `file <PATH>` | Compute the hash of a file (`--algorithm md5\|sha256`) |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (missing file, unreadable file, ...)

## Development

```bash
cargo test -p asta
cargo clippy -p asta --all-targets -- -D warnings
cargo fmt -p asta -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
