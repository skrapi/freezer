# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Freezer is a Linux-only CLI tool (Rust, edition 2024) that maintains a list of RSS/Atom
feeds and emails a plaintext digest of articles published within a configurable time
window. The published binary is `freezer`.

## Commands

```bash
cargo build                       # build
cargo run -- <subcommand>          # run locally (Add | Remove | List | Publish)
cargo test                         # run all tests
cargo test test_plaintext          # run a single test by name
cargo fmt --check                  # formatting check (CI enforces this)
cargo clippy -- -D warnings        # lint; CI treats warnings as errors
cargo install --path .             # install the binary
```

CI (`.github/workflows/general.yml`) runs test, fmt, clippy, and llvm-cov coverage on
every push/PR to `main`. `audit.yml` runs `cargo deny check advisories` daily and on any
`Cargo.toml`/`Cargo.lock` change.

## Architecture

`main.rs` is the binary (clap subcommand dispatch + `send_digest` SMTP helper); everything
else is the library exposed through `lib.rs`. Data flows:

1. **Configuration** (`configuration.rs`) — loaded at startup from
   `~/.config/freezer/freezer.toml`. `Configuration` is the real on-disk shape: nested
   `[subscriber]` and `[sender]` tables. Note the README's "minimal config" snippet shows
   flat fields — that flat shape matches only `Subscriber::from_config_file` /
   `tests/config.toml`, not the actual runtime config. A real config needs both tables,
   `[sender]` carrying `app_email` / `app_password` for SMTP auth.
2. **Subscriber** (`subscriber.rs`) — holds the feed list and time window. `feeds` is a
   `HashMap<url, date_added>` but TOML stores it as an array of `[url, date]` pairs;
   `serialize_hashset`/`deserialize_hashset` bridge the two and sort on save for stable
   diffs. `collect_all_feeds` fetches every feed concurrently (`reqwest` + `join_all`) and
   parses with `feed-rs`.
3. **Feeds** (`feeds.rs`) — wraps parsed `feed-rs` feeds. `get_new_entries` filters by
   publish date; `SimpleEntry` is the flattened (title, link, date) form used downstream.
4. **Digest** (`digest.rs`) — turns `SimpleEntry` list into the plaintext email body.

`Publish` ties it together: compute `since` from `time_period_hours` → fetch feeds → new
entries → `SimpleEntry`s → `Digest` → `send_digest` (lettre SMTP relay to
`smtp.fastmail.com`).

## Conventions

- Tests read fixtures from the `tests/` directory (`rss.xml`, `atom.xml`, `config.toml`)
  with paths relative to the crate root.
- Library code currently favors `.unwrap()`/`.expect()` over error propagation,
  particularly in feed fetching/parsing. Match the surrounding style unless deliberately
  hardening a path.
- The sender address (`freezer@sylvansmit.com`) and digest sign-off (`Sylvan`) are
  hardcoded in `main.rs`/`digest.rs`.
