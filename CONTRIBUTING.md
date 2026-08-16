# Contributing to cgmath-next

Thanks for considering contributing. This project has an unusual
constraint compared to most crates: it is a *soundness-focused,
source-compatible fork* of an existing, widely-used library, not a
greenfield project. Please read this before opening a PR — it will save
you a rejected PR for reasons that aren't obvious from the code alone.

## The project's priorities, in order

1. Eliminate safe-Rust-reachable undefined behavior.
2. Let `cgmath` 0.18.0 users migrate with a one-line `Cargo.toml` change.
3. Preserve numeric results, types, traits, features, and serialization
   formats wherever possible.
4. Keep remaining `unsafe` code auditable.
5. Long-term maintainability: CI, tests, docs, release process.
6. New features and large redesigns are **explicitly deferred** until
   after the initial compatibility release. See `AGENTS.md` in this
   repository for the full policy and current phase.

If your PR is a new feature, a performance rewrite, an API redesign, or a
dependency major-version bump, please open an issue first — it will very
likely need to wait for a later release series regardless of quality.

## What's welcome right now

* Soundness fixes (with a Miri regression test that fails before the fix
  and passes after)
* Bug reports and fixes that don't change public API
* Test coverage, especially differential tests against real `cgmath`
  0.18.0 output, or reverse-dependency migration fixtures
  (`compat/fixtures/`)
* Documentation fixes, including in `docs/unsafe-audit.md` if you can
  verify or improve a safety invariant
* CI/tooling improvements that don't change what's built

## Before you open a PR

* Run `cargo test` and `cargo test --all-features` — both must pass with
  the same counts as `docs/baseline.md` plus your change.
* If you touched `unsafe` code, run the relevant tests under Miri:
  `cargo +nightly miri test --test <your test>`, and update
  `docs/unsafe-audit.md`.
* If you touched public API in any way (even additively), regenerate the
  API inventory (see `docs/api-inventory.md` for the exact commands) and
  include the result in your PR description — API changes need explicit
  sign-off per `AGENTS.md`, not silent approval.
* Don't mix formatting-only changes with functional changes in the same
  commit. `cargo fmt --check` currently fails on several files inherited
  from upstream (tracked in `docs/baseline.md`) — please don't
  bulk-reformat them in an unrelated PR.
* Don't loosen `approx` tolerances to make a failing numeric test pass;
  if a numeric result changed, that's the bug to explain, not paper over.

## Commit style

Small, focused commits. See the existing history for the pattern this
project follows (e.g. `test: reproduce X` before `fix: X`, `docs: ...`
separate from functional changes).

## Reporting soundness issues

Please see [`SECURITY.md`](SECURITY.md) rather than a public issue —
undefined-behavior repro cases shouldn't be posted publicly before a fix
ships.
