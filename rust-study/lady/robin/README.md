# robin

File utility CLI for the `lady` workspace: file hashes, file info and
directory trees.

## Features

- MD5 hash of a file (streamed in 8 KiB chunks via the shared `hash-utils` crate)
- Detailed file information (type, size, permissions)
- Image file information by extension
- Directory tree with sizes, depth-limited, hidden entries skipped
- Human-readable byte formatting
- Shell completion generation

## Installation

```bash
cargo install --path robin
# or from the workspace root
cargo build --release -p robin
```

## Usage

```bash
robin md5 data.bin            # File / Size / MD5 / Elapsed report
robin info data.bin           # type, size, read-only
robin img photo.png           # type, size, read-only
robin tree src/               # tree with file sizes (max depth 3)

# Demo command kept from the original project
robin add --num 42

# Shell completions
robin completions --shell powershell
```

## Commands

| Command | Description |
| --- | --- |
| `add --num <N>` | Demo command: echoes the given number |
| `md5 <FILE>` | Compute the MD5 hash of a file |
| `img <FILE>` | Show image file information |
| `info <FILE>` | Show detailed file information |
| `tree <DIR>` | Display directory tree |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (missing file, not a directory, unreadable file)

## Development

```bash
cargo test -p robin
cargo clippy -p robin --all-targets -- -D warnings
cargo fmt -p robin -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
