# MSRV (Minimum Supported Rust Version)

**Measured: Rust 1.71.0**, as of 2026-08-16, against today's default
(unpinned) dependency resolution. Declared in `Cargo.toml` as
`rust-version = "1.71"`.

## Method (AGENTS.md §13)

1. Deleted `Cargo.lock` and ran `cargo +<toolchain> check --all-features`
   / `cargo +<toolchain> test --all-features` against a sequence of
   `rustup`-installed toolchains, bisecting on actual compile/test
   results rather than guessing:

   | Toolchain | Result |
   |---|---|
   | 1.56.0 | fails — `libc v0.2.189` (pulled in transitively via `rand`/`getrandom`) requires rustc 1.65+ |
   | 1.65.0 | fails — `proc-macro2 v1.0.107` requires rustc 1.71+ |
   | 1.70.0 | fails — `quote v1.0.47` requires rustc 1.71+ |
   | **1.71.0** | **passes** — `cargo test --all-features`: 300 tests, 0 failed (all 13 binaries, same as the current-stable baseline in `docs/baseline.md`) |
   | 1.74.0 | passes (`cargo check --all-features`) |
   | current stable (1.97.0) | passes (used throughout this session) |

2. `rust-version = "1.71"` added to `Cargo.toml` and re-verified to build
   cleanly under current stable afterward.

## What this number actually measures

**This is not cgmath-next's own source code's language-feature floor.**
Every failure above was a *transitive* dependency (`libc`, `proc-macro2`,
`quote` — all pulled in by `serde_derive`/`rand`'s macro or randomness
internals, not referenced directly by `cgmath-next`'s own code) declaring
its own MSRV, not a compile error in `src/`. `cgmath-next`'s direct
dependencies (`approx = "0.4"`, `mint = "0.5"`, `num-traits = "0.2"`,
`rand = "0.8"`, `serde = "1.0"`) are unpinned semver ranges, so `cargo
check` resolves to whatever the *latest* compatible version of each is
today — and those latest versions' own transitive tooling has raised its
MSRV over the ~5 years since `cgmath` 0.18.0 shipped (Jan 2021).

This means:

* **1.71 is today's practical floor for `cargo build` with no extra
  steps**, and is what's recorded in `Cargo.toml`/CI.
* It is very likely possible to go meaningfully lower (potentially back
  toward 1.56 or below) by pinning older, still-semver-compatible
  transitive versions with `cargo update -p <pkg> --precise <version>`
  and committing the resulting `Cargo.lock` for MSRV CI specifically.
  **This was not attempted this session** — it's listed as a candidate
  follow-up in `docs/release-checklist.md` rather than done speculatively,
  since it would need its own verification pass (does the pinned old
  `serde`/`rand` chain still work correctly, does it have its own
  soundness issues, etc.) and this session's priority was the soundness
  fix, not MSRV minimization.
* **This number will drift upward again** the next time `cargo update`
  runs, purely from the ecosystem's transitive MSRV creep, independent of
  any change to `cgmath-next` itself. Re-measure whenever dependency
  versions change, per this same procedure.

## Per AGENTS.md §21

This is a *new* MSRV declaration, not a *raise* of an existing one —
`cgmath` 0.18.0 (published Jan 2021, before Cargo's `rust-version` field
existed in common use) never declared an MSRV at all. Recorded here rather
than treated as a stop-and-report item for that reason. If a future
pinned-transitive-deps pass lowers this number, or if ecosystem drift
raises it again, both are expected maintenance, not policy decisions.
