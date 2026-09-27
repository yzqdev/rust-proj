# pela

A fictional versioning CLI for the `lady` workspace — the **clap derive API**
variant of the same CLI that `sampo` implements with the clap **builder API**.
Both are kept as teaching examples of the two clap styles.

Operations are simulated (they print what would happen); `init` and `clone`
do create directories.

## Features

- Git-like command surface: `init`, `clone`, `diff`, `push`, `add`,
  `commit`, `status`, `stash`, `log`
- `stash` with nested subcommands (`push`, `pop`, `apply`, `list`)
- External subcommand passthrough
- Shell completion generation

## Installation

```bash
cargo install --path pela
# or from the workspace root
cargo build --release -p pela
```

## Usage

```bash
pela init my-project          # creates ./my-project/ with a README
pela clone https://example.com/group/repo.git
pela add src/main.rs
pela commit -m "first commit"
pela status
pela log -n 5
pela stash push -m "wip"
pela stash list
pela diff HEAD~1 --color=always

# External subcommand passthrough
pela remote add origin

# Shell completions
pela completions --shell fish
```

## Commands

| Command | Description |
| --- | --- |
| `init <NAME>` | Initialize a new repository |
| `clone <REMOTE>` | Clone a repository |
| `diff [BASE] [HEAD] [-- PATH]` | Compare two commits or files |
| `push <REMOTE>` | Push changes to a remote |
| `add <PATH>...` | Stage files |
| `commit -m <MESSAGE>` | Commit staged changes |
| `status` | Show working tree status |
| `stash [push\|pop\|apply\|list]` | Stash changes |
| `log [-n COUNT]` | Show commit history |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (missing path, filesystem failure)

## Development

```bash
cargo test -p pela
cargo clippy -p pela --all-targets -- -D warnings
cargo fmt -p pela -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
