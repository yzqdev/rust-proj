# Minigrep

A fast and simple grep-like text search tool.

## Requirements

[Rust-lang](https://www.rust-lang.org/en-US/install.html) should be installed.

## Installation

```sh
git clone https://github.com/subhojit777/minigrep.git
cd minigrep
cargo build --release
```

## Usage

```sh
minigrep [OPTIONS] <QUERY> [FILE]...
```

Example - `minigrep --insensitive nemo find-nemo-the-movie.txt`

### Options

| Option | Description |
| --- | --- |
| `-i, --insensitive` | Case-insensitive search |
| `-w, --exact-word` | Whole-word (exact) match |
| `-c, --count` | Count matches instead of printing them |
| `-n, --line-number` | Show line numbers |

If no file is given, input is read from stdin.
Matches are highlighted; when searching multiple files the file name is
printed as a prefix.

## Documentation

```sh
cargo doc --no-deps --open
```

## Testing

```sh
cargo test
```

## License

Dual licensed under MIT and Apache-2.0 (see LICENSE-MIT / LICENSE-APACHE).
