# Baseline: unmodified cgmath 0.18.0

Run against the faithful import commit (`chore: import cgmath 0.18.0 release
source`), before any renaming or fixes, on:

* `rustc 1.97.0 (2d8144b78 2026-07-07)` / `cargo 1.97.0` (stable, macOS
  aarch64)
* `cargo fmt` from the same toolchain
* `cargo clippy` from the same toolchain

Raw stdout/stderr for every command is saved under `baseline-results/*.log`.

| Command | Exit | Result | Classification |
|---|---|---|---|
| `cargo check` | 0 | builds, 1122 warnings (862 duplicates) | toolchain-update failure (warnings only, see below) |
| `cargo test` | 0 | 278 passed, 0 failed | pass |
| `cargo test --all-features` | 0 | 281 passed, 0 failed | pass |
| `cargo check --no-default-features` | 0 | builds, same warning class | toolchain-update failure (warnings only) |
| `cargo test --doc` | 0 | 22 passed, 0 failed | pass |
| `cargo fmt --check` | 1 | 11 files need reformatting | toolchain-update failure (rustfmt style rules drifted since 2021) |
| `cargo clippy --all-targets --all-features` | 101 | 454 warnings; **4 benches fail to compile** | mixed — see below |

## `cargo check` / `--no-default-features` warnings (1122 total, non-blocking)

Dominated by one macro-generated warning repeated per generated impl:
`#[inline] attribute cannot be used on macro calls` (a `impl_operator!`
macro-hygiene warning that did not exist when 0.18.0 shipped — 2021-era rustc
accepted it, current rustc warns and says it becomes a hard error later).
This is **toolchain drift**, not a `cgmath-next` regression: it is present in
the untouched upstream source under current stable rustc. Left unfixed in the
baseline import per AGENTS.md §15 ("baseline warning を記録し、新規 warning
を禁止する" — record now, do not mass-fix in the import commit).

## `cargo fmt --check` (11 files, exit 1)

Upstream 0.18.0 was formatted with a 2020-era rustfmt. Current stable rustfmt
disagrees on 11 files. Toolchain-update failure, not a regression — no
functional issue. Left unformatted in the baseline import; will not be
bulk-reformatted in the same commit as any functional change (AGENTS.md §22).

## `cargo clippy --all-targets --all-features` (exit 101)

Two distinct issues bundled in one exit code:

1. **454 clippy warnings** on lib/tests — dependency-resolution/toolchain
   drift (newer clippy, stricter default lints than existed in 2021). Top
   categories: `#[inline]` on macro calls (248), deprecated `cfg_attr` for
   rustfmt (42), redundant field names (32), elidable lifetimes (31),
   `From`-preferred-over-`Into` (29), `Copy`-type `.clone()` (20). None
   indicate a correctness bug; all are style/idiom lints. Toolchain-update
   failure class.
2. **4 benches fail to compile** (`bench "quat"`, `"mat"`, `"construction"`,
   `"vec"`): `error[E0554]: #![feature] may not be used on the stable release
   channel`. `benches/*.rs` use `#![feature(test)]` + `extern crate test`,
   which is the unstable native bench harness — it requires a nightly
   toolchain and always did. This is an **upstream baseline characteristic**,
   not a regression from this fork: the benches were never runnable on
   stable. Re-classified as "upstream baseline failure" rather than
   toolchain drift, since even 2021-era stable rustc would reject
   `#![feature(test)]`.

Because of (2), `cargo clippy --all-targets` cannot succeed on stable at all,
independent of `cgmath-next`. CI will need to either run clippy without
`--all-targets` on stable, or run the bench-covering clippy pass on nightly
only. Tracked as a release-blocker candidate for Phase 8 CI design, not fixed
here (fixing the benches to run on stable would mean rewriting them to not
use the unstable harness, which is a functional change to committed-as-is
upstream source and out of scope for the faithful-import phase).

## Conclusion

No regressions introduced by this fork (none exist yet — this is the
unmodified baseline). All non-zero exits are either upstream-vs-toolchain
drift (warnings, fmt) or an upstream characteristic that predates this fork
(nightly-only benches). None require paving over per AGENTS.md §21 — none
involve public API, soundness, licensing, numeric results, or serialization.
