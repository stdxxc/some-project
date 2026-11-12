# AGENTS.md

Compact guidance for working in this repo. Keep it factual and verify before trusting.

## What this is

Rust binary crate (`some-project`), currently a hello-world skeleton intended to grow incrementally. No dependencies, CI, or license yet. Remote: `git@github.com:stdxxc/some-project.git` (public).

## Environment (verified 2026-11-12)

- Repo lives in WSL Ubuntu 26.04 at `/home/stefa_ubuntu/some-project`; the Windows UNC view is `\\wsl.localhost\Ubuntu\home\stefa_ubuntu\some-project`.
- **Rust is not installed yet** — `cargo`/`rustup` are absent. Install when needed:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
  `rust-toolchain.toml` pins stable and adds `rustfmt` + `clippy`.
- Git 2.53 installed. Commit identity: `stdxxc <stdxxc@users.noreply.github.com>`. GitHub SSH key (`~/.ssh/id_ed25519`) is registered and authenticates as `stdxxc`.
- Prefer running commands through WSL (`wsl -d Ubuntu -- bash -lc '...'`) rather than PowerShell; PowerShell strips `$VAR` in nested Bash snippets.
- The repo is edited through a Windows UNC path, so keep LF endings. `.gitattributes` enforces `eol=lf`; don't rewrite files as CRLF.

## Commands (after Rust is installed)

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

- `src/main.rs` — binary entrypoint; add modules as `src/<name>.rs` + `mod` declarations.
- `Cargo.toml` — manifest. Binary crate: commit `Cargo.lock`.
- `.gitignore` ignores `/target`.

## Conventions

- Stay std-only until a dependency is clearly justified.
- When logic needs isolated tests, split it into `src/lib.rs` and keep `main.rs` thin.

## Not set up yet

CI workflow, LICENSE.
