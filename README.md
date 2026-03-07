# git-digest

Rust CLI tool for generating a simple “digest” of a repository: a directory tree and/or concatenated file contents.

## Requirements

- Rust toolchain (stable) with Cargo

## Setup

Clone the repo and build:

```bash
git clone https://github.com/iktrnch/git-digest.git
cd git-digest
cargo build --release
```

Optionally install the CLI into your PATH:

```bash
cargo install --path .
```

## Usage

Run via Cargo (from this repo):

```bash
cargo run -- --help
```

Or, if installed, run:

```bash
git_digest --help
```

### Common examples

Print a directory tree (default directory is `.`):

```bash
git_digest --tree
```

Print concatenated file contents:

```bash
git_digest --files
```

Digest a specific directory:

```bash
git_digest -d path/to/repo --tree --files
```

Use a custom ignore file (defaults to `./.gitignore`):

```bash
git_digest -i path/to/ignorefile --tree
```

## Notes

- Ignore matching is currently simple substring matching (plus an automatic `.git` ignore).
- Output is written to stdout.
