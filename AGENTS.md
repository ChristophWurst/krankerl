# AGENTS.md - Krankerl

## Project Overview

Krankerl is a Rust CLI tool that helps developers manage [Nextcloud](https://nextcloud.com/)
apps: scaffolding config, packaging release archives, signing them, publishing to the
Nextcloud App Store, and a few `occ` shortcuts for local development.

- **Crate name:** `krankerl`
- **Entry point:** `src/main.rs` (parses CLI args with `docopt`, dispatches to `krankerl::commands::*`)
- **License:** see `LICENSE`

## Build Commands

```bash
cargo build                 # Debug build
cargo build --release       # Release build (used by the release workflow)
```

## Lint / Static Analysis

There is no lint/static-analysis step configured in CI or locally (no `rustfmt.toml`,
no clippy invocation in the GitHub workflows). If you run `cargo fmt` or `cargo clippy`
yourself, match the existing code style rather than introducing new formatting wholesale.

## Test Commands

```bash
cargo test                          # Run all unit + integration tests
cargo test <substring>              # Run tests whose name matches <substring>
cargo test --package krankerl config::app::tests::test_parse_config_with_commands  # Single test
```

Tests live inline in `#[cfg(test)] mod tests` blocks next to the code they cover
(e.g. `src/config/app.rs`, `src/packaging/pipeline.rs`), not in a separate `tests/`
directory of `*.rs` files. `tests/apps/app1-3` are fixture Nextcloud apps used by
these tests, and `tests/assets/minimalist.tar.gz` is a packaged fixture app used by
the packaging pipeline tests. Fixture apps are copied into a `TempDir` before each
test mutates them (see `prepare_fs_test` in `src/config/app.rs`), so tests don't
touch the checked-in fixtures.

## Project Architecture

```
src/
  main.rs                   # docopt CLI definition + dispatch to commands
  lib.rs                    # crate root, re-exports commands/config/occ/packaging, publish_app()
  commands/                 # one file per CLI subcommand, thin wrappers around config/packaging/occ
    enable.rs, disable.rs   #   `occ app:enable|disable` shortcuts (assumes occ is at ../../occ)
    init.rs                 #   scaffolds krankerl.toml + .nextcloudignore
    package.rs               #   packages the current app (see packaging/ below)
    sign_package.rs          #   signs build/artifacts/<id>.tar.gz with ~/.nextcloud/certificates/<id>.key
    login.rs                 #   stores appstore/github tokens via config::krankerl
    up.rs                     #   runs `npm install`+build and `composer install` in parallel threads
    version.rs               #   bumps appinfo/info.xml version (major/minor/patch)
    clean.rs                  #   removes build/artifacts
  config/
    krankerl.rs               # global CLI config (appstore/github tokens), stored as JSON under XDG config dir
    app.rs                     # per-app krankerl.toml ([package] before_cmds), parsed with `toml`
    reader.rs                  # ConfigReader trait, abstracts file reads for testability
  packaging/
    pipeline.rs                # typestate pipeline: App -> ClonedApp -> AppWithDependencies -> BuiltApp -> AppArchive | ShippedApp
    artifacts.rs                # git-clones the app into a tempdir, clears build/artifacts
    commands.rs                 # runs `before_cmds` from krankerl.toml via `sh -c`
    archive.rs                   # builds the final .tar.gz respecting .nextcloudignore
  occ.rs                      # thin wrapper around the Nextcloud `occ` binary
```

### Packaging pipeline (typestate pattern)

`krankerl package` does not operate on the current working tree directly. It drives
the app through a sequence of owned, consuming states defined in `src/packaging/pipeline.rs`:

1. `App` — holds the source path.
2. `.clone()` -> `ClonedApp` — reads `appinfo/info.xml` via `nextcloud_appinfo`, then
   `git clone`s the source path into a `TempDir` so packaging is independent of
   uncommitted local changes.
3. `.install_dependencies()` -> `AppWithDependencies` — currently a no-op placeholder.
4. `.build()` -> `BuiltApp` — loads `krankerl.toml` (falls back to `AppConfig::default()`
   if absent) and runs its `package.before_cmds` inside the cloned tree.
5. `.into_archive()` -> `AppArchive` (default) or `.into_shipped()` -> `ShippedApp`
   (`--shipped` flag) — walks the cloned tree with the `ignore` crate honoring
   `.nextcloudignore`, and either tars+gzips it to `build/artifacts/<id>.tar.gz` or
   copies the files to `build/artifacts/<id>/`.

Each stage method consumes `self` and returns the next state, so a given `App` can
only move forward through the pipeline — this is enforced by the type system, not
by runtime checks.

### Other things worth knowing

- Global config (`appstore_token`, `github_token`) lives in a JSON file under the
  XDG config dir (`~/.config/krankerl/config.json` on Linux), managed by `src/config/krankerl.rs`.
- Per-app config (`krankerl.toml`) is optional; its absence just means default
  (empty) packaging behavior.
- `publish_app` (in `lib.rs`) and the appstore-token flow delegate to the external
  `nextcloud_appstore` / `nextcloud_appsignature` / `nextcloud_appinfo` crates — most
  Nextcloud-specific domain logic (info.xml parsing, signing, app store API) lives
  in those crates, not in this repo.
- `enable`/`disable` commands assume they're run from inside an app directory two
  levels below a Nextcloud install root, so `occ` can be found at `../../occ`.

## CI/CD

GitHub Actions workflows (`.github/workflows/`):
- `build.yml` — `cargo build` on PRs
- `test.yml` — `cargo test` on PRs
- `conventional_commits.yml` — enforces conventional commit format on PRs to `master`
- `tag-release.yml` — scheduled job that bumps the version (`cargo-bump`, via `build/pre-commit.js`)
  and generates the changelog from conventional commits
- `release.yml` — on `v*.*.*` tags, builds a release binary and a `.deb` package
  (via `cargo-deb`) and attaches them to a GitHub release
- `dependabot-approve.yml` — auto-approves/merges minor Dependabot PRs

## Conventional Commits

This project enforces conventional commit messages, e.g.:

```
feat: add new feature
fix: correct bug in packaging pipeline
chore: update dependencies
```

## Agent-generated commits, PRs and comments

Do not add "Generated with", "Co-Authored-By", tool/model attribution, or similar
advertisement lines to commit messages, PR descriptions, or PR/issue comments.
