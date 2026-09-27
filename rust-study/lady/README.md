# lady

Rust CLI workspace containing a set of small, focused command-line tools,
each in its own crate. All crates share edition 2024, workspace-level
dependencies and lints, and follow a `lib` + `bin` split so the core logic
is unit-testable and the binary stays a thin entry point.

## Projects

| Crate | Type | Description |
| --- | --- | --- |
| [asta](asta/) | CLI | Hash calculator (MD5 / SHA256 for strings and files) |
| [httpman](httpman/) | CLI | HTTP client (`get`/`post`/`put`/`delete`/`head`/`patch`, JSON body, headers, pretty output) |
| [taoqi](taoqi/) | CLI | JSON processor (`format` / `validate` / `minify` / `query`) |
| [phoebe](phoebe/) | CLI | CSV → JSON converter + package installer simulation |
| [robin](robin/) | CLI | File utilities (MD5, file info, image info, directory tree) |
| [pela](pela/) | CLI (teaching) | Fictional versioning CLI — clap **derive API** variant |
| [sampo](sampo/) | CLI (teaching) | Fictional versioning CLI — clap **builder API** variant (same CLI as `pela`) |
| [hash-utils](hash-utils/) | library | Shared streaming MD5/SHA256 helpers used by `asta` and `robin` |

Every CLI supports `--help`, `--version` and a `completions --shell <SHELL>`
subcommand (bash / zsh / fish / PowerShell / elvish).

## Build

```bash
cargo build --workspace            # debug
cargo build --workspace --release  # optimized, stripped binaries in target/release/
```

## Test

```bash
cargo test --workspace
```

Each project ships unit tests (core logic) and CLI integration tests
(`assert_cmd` + `predicates`) covering `--help`, `--version`, invalid
commands, and the core commands against temporary files.

## Quality gates

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Installing a tool

```bash
cargo install --path asta    # or any other member crate
```

## Structure conventions

- `src/lib.rs` — CLI definition + business logic, returns `Result<String, Error>`
- `src/main.rs` — thin entry point: parse → run → print stdout / stderr, exit code
- `tests/cli.rs` — CLI integration tests
- errors are user-friendly (`thiserror` / `anyhow`) with the error chain preserved

## License

Unspecified; personal study workspace (yzqdev).
