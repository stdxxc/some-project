# AGENTS.md

Compact guidance for working in this repo. Keep it factual and verify before trusting.

## What this is

Rust binary crate (`some-project`), currently a hello-world skeleton intended to grow incrementally. No dependencies, CI, or license yet. Remote: `git@github.com:stdxxc/some-project.git` (public).

## Environment (verified 2026-11-12)

- Repo lives in WSL Ubuntu 26.04 at `/home/stefa_ubuntu/some-project`; the Windows UNC view is `\\wsl.localhost\Ubuntu\home\stefa_ubuntu\some-project`.
- Rust 1.99.1 installed via rustup (`cargo`, `rustfmt`, `clippy`). `~/.profile` sources `$HOME/.cargo/env`; non-login shells may need `. "$HOME/.cargo/env"` first.
- C linker: `gcc`/`cc` 15.2 (GCC 15). Without it, `cargo test` fails with `linker cc not found`.
- Host C: drive is nearly full (~2.8 GB free). WSL failed to start once (`Wsl/Service/CreateInstance/E_FAIL`) because it could not allocate swap; `~/.wslconfig` sets `swap=0`. Free space before large installs.
- Git 2.53 installed. Commit identity: `stdxxc <stdxxc@users.noreply.github.com>`. GitHub SSH key (`~/.ssh/id_ed25519`) is registered and authenticates as `stdxxc`.
- Prefer running commands through WSL (`wsl -d Ubuntu -- bash -lc '...'`) rather than PowerShell; PowerShell strips `$VAR` in nested Bash snippets.
- The repo is edited through a Windows UNC path, so keep LF endings. `.gitattributes` enforces `eol=lf`; don't rewrite files as CRLF.
- Files created through the Windows UNC path can end up owned by `root`. If writes fail with "Permission denied", run `sudo chown -R stefa_ubuntu:stefa_ubuntu ~/some-project`.

## Commands

```sh
cargo build            # compile
cargo run              # run the binary
cargo test             # all tests
cargo test <name>      # single test by substring; add -- --exact for exact
cargo fmt              # format
cargo clippy --all-targets -- -D warnings
```

Verification order: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`.

## Layout

- `src/main.rs` - binary entrypoint; add modules as `src/<name>.rs` + `mod` declarations.
- `Cargo.toml` - manifest. Binary crate: commit `Cargo.lock`.
- `.gitignore` ignores `/target`.

## Conventions

- Stay std-only until a dependency is clearly justified.
- When logic needs isolated tests, split it into `src/lib.rs` and keep `main.rs` thin.

## Not set up yet

CI workflow, LICENSE.
