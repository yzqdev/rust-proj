# sampo

A fictional versioning CLI for the `lady` workspace — the **clap builder API**
variant of the same CLI that `pela` implements with the clap **derive API**.
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
cargo install --path sampo
# or from the workspace root
cargo build --release -p sampo
```

## Usage

```bash
sampo init my-project          # creates ./my-project/ with a README
sampo clone https://example.com/group/repo.git
sampo add src/main.rs
sampo commit --message "first commit"
sampo status
sampo log -n 5
sampo stash push --message "wip"
sampo stash list
sampo diff HEAD~1 --color=always

# External subcommand passthrough
sampo remote add origin

# Shell completions
sampo completions --shell zsh
```

## Commands

| Command | Description |
| --- | --- |
| `init <NAME>` | Initialize a new repository |
| `clone <REMOTE>` | Clone a repository |
| `diff [BASE] [HEAD] [-- PATH]` | Compare two commits or files |
| `push <REMOTE>` | Push changes to a remote |
| `add <PATH>...` | Stage files |
| `commit --message <MESSAGE>` | Commit staged changes |
| `status` | Show working tree status |
| `stash [push\|pop\|apply\|list]` | Stash changes |
| `log [-n COUNT]` | Show commit history |
| `completions --shell <SHELL>` | Generate shell completions |

## Exit codes

- `0` — success
- `1` — runtime error (missing path, filesystem failure)

## Development

```bash
cargo test -p sampo
cargo clippy -p sampo --all-targets -- -D warnings
cargo fmt -p sampo -- --check
```

## License

Unspecified; internal study project of the `lady` workspace.
