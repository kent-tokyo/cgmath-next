# Release checklist

Tracks AGENTS.md §20's completion conditions. Update this file as items
change status -- it's the single source of truth for "are we ready to cut
a release," not a snapshot to be trusted stale.

**`0.18.1-alpha.1` is published to crates.io** (2026-08-16), tagged
(`v0.18.1-alpha.1`, commit `e6c07a0`), and released on GitHub as a
pre-release. `0.18.1` stable has **not** been published -- no tag, no
GitHub Release, nothing beyond the alpha exists yet. `crates.io` publish,
tag creation, and GitHub Release creation remain AGENTS.md §21
stop-and-report items requiring explicit human approval each time; the
alpha.1 approval already exercised does not carry forward to stable.

## 0.18.1-alpha.1

| # | Condition | Status |
|---|---|---|
| 1 | Provenance of the published 0.18.0 is recorded | done -- `docs/provenance.md` |
| 2 | Apache-2.0 attribution is preserved | done -- `LICENSE`, per-file copyright headers unmodified |
| 3 | Upstream tests pass | done -- 256/256 original tests pass unmodified, `docs/baseline.md`; benchmark baseline (§16) also now recorded there, 59/59 pass |
| 4 | `swap_columns`' known UB is fixed | done, and broader than the ticket -- see the fix commit, `docs/unsafe-audit.md` |
| 5 | Miri regression tests pass | done -- `tests/soundness/`, 22/22 pass under `cargo +nightly miri test --test soundness` |
| 6 | Public API diff has been generated | done -- `docs/api-inventory.md`, zero-diff result |
| 7 | Feature matrix passes | done -- `docs/compatibility.md`, 6/6 individual rows + all-features + all 6 pairwise combinations |
| 8 | 3+ migration fixtures pass | done, and exceeds the stable gate too -- `compat/fixtures/reverse-deps/RESULTS.md`, 5 real reverse-dependency crates |
| 9 | Unsafe inventory is complete | done -- `docs/unsafe-audit.md`; was 4 pattern groups, now 2 (`UNSAFE-001`, `002`) after `UNSAFE-003` was resolved into safe code and `UNSAFE-004`'s dead SIMD files were deleted this session |
| 10 | Each unsafe has an audit status recorded | done -- same doc, per-group Status field |
| 11 | README, CHANGELOG, SECURITY.md exist | done |
| 12 | crates.io publish has not happened yet | done (trivially true) |

**All 12 alpha conditions are met as of this checklist's last update.**
`0.18.1-alpha.1` could be tagged/published pending explicit human
approval (§21) -- this document does not constitute that approval.

## 0.18.1 stable

| # | Condition | Status |
|---|---|---|
| 1 | Zero unexplained public API removals | met -- zero API diff |
| 2 | Zero known safe-to-UB paths | **guarded and accepted, not literally zero.** `UNSAFE-002` (tuple-layout transmute) runs a runtime layout check (size/align/per-field offset) before every transmute in this category, panicking instead of transmuting on a mismatch; verified via Miri, a negative-control test, and release-build disassembly confirming zero cost when layout matches (see `docs/unsafe-audit.md`'s feasibility-study section). This converts the failure mode from silent UB to a loud panic -- it is **not** a language-level soundness proof, tuple layout remains officially unspecified. **This is now a permanent, explicitly accepted known limitation of the `0.18.x` stable series**, not an open item pending further work: closing it fully would require removing the reference-returning conversions, a public API change that's out of scope for `0.18.x` by the series' own compatibility policy (see `CONTRIBUTING.md`). Accepting this row as "guarded" rather than "met" is a deliberate release decision, recorded here for the stable release, not a gap someone forgot to close |
| 3 | Remaining unsafe invariants are documented | met -- `docs/unsafe-audit.md` |
| 4 | Release-targeted unsafe tests pass under Miri | **met.** `UNSAFE-001` has a dedicated Miri regression suite (`tests/soundness/array_conversions.rs`, 14 tests, all `AsRef`/`AsMut`/`From<&[..]>`/`From<&mut [..]>` paths across `Vector1-4`/`Point1-3`/`Matrix2-4`/`Quaternion`, including write-back-through-the-view checks and `-Zmiri-strict-provenance`); `UNSAFE-002` already has its own Miri coverage (see row 2); `UNSAFE-003` and `UNSAFE-004` are resolved/deleted, so there is nothing unsafe left in either category to test under Miri |
| 5 | serde, mint, rand, swizzle compatibility is verified | **met.** `serde` and `mint` have dedicated differential tests against real `cgmath` 0.18.0 (`compat/fixtures/dual-dep/`, 11/11 pass, byte-exact for serde and orientation-verified for mint); `rand`'s `Distribution` impls are confirmed byte-identical to pristine 0.18.0 source and have a dedicated contract test (`tests/rand_distribution.rs`, 6/6 pass); `swizzle`'s full 550-method inventory is confirmed byte-identical to pristine 0.18.0 with the feature on and empty with it off, plus a compile-fail fixture (`compat/fixtures/swizzle-off/`); `serde`/`mint`/`rand` all have confirmed feature-off dependency-graph leak checks. The `dual-dep` and `swizzle-off` fixtures now also run as a blocking `compat` CI job on every push/PR, so this evidence is continuously regression-checked, not just point-in-time. See `docs/compatibility.md` for the full detail behind each -- this is stronger than "pairwise `cargo test` passes", which was the prior state |
| 6 | 5+ migration fixtures pass | met -- 5/5 (`arcball`, `crevice`, `truck-base`, `vector-traits`, `three-d`) |
| 7 | MSRV is measured and documented | met -- `docs/msrv.md`, though it documents that the number is driven by transitive deps and will drift |
| 8 | CI passes on 3 platforms | met -- verified by real runs against `origin/main`, and it's caught 2 real issues so far (not just "ran green"): [run](https://github.com/kent-tokyo/cgmath-next/actions/runs/31937609325) 1 found a `cargo audit` lockfile bug (fixed, `825d7f5`); the [layout-guard commit's run](https://github.com/kent-tokyo/cgmath-next/actions/runs/31940302025) found the `miri` job's `--lib` sweep hitting Miri's own known float non-determinism in `slerp` for the first time (fixed by narrowing that step's scope, not by weakening any test, `f155eaf`; [confirmed green](https://github.com/kent-tokyo/cgmath-next/actions/runs/31940574715) after). Otherwise green across all blocking jobs except the pre-documented informational `fmt` job |
| 9 | `cargo audit` and `cargo deny` results are explained | met -- `deny.toml` added, `cargo deny --all-features check` run clean (advisories/bans/licenses/sources all ok); `security` CI job wires this in going forward |
| 10 | Known compatibility gaps are published | met -- `docs/compatibility.md`, `docs/unsafe-audit.md` both list open gaps |
| 11 | Release checklist is complete | 10/12 rows fully "met" as of this update. Row 2 (`UNSAFE-002`) is "guarded" by design, not "met" -- tuple layout is language-unspecified and no amount of further work in this repo can change that, only removing the reference-returning conversions (a public API change) would, and that's explicitly out of scope. Row 12 (human publish approval) is intentionally still pending -- this document doesn't grant it, a human does |
| 12 | Explicit human approval obtained before publish | pending -- not requested yet |

## Outstanding work before stable, in priority order

1. ~~Resolve or formally accept `UNSAFE-002`'s risk~~ -- a runtime
   layout guard was implemented and verified (feasibility study
   approved and completed), see row 2 above and
   `docs/unsafe-audit.md`. Converts silent UB to a detected panic;
   does not make tuple layout language-guaranteed, which remains
   impossible without removing the reference-returning conversions
   (a public API change, still out of scope).
2. ~~1 more reverse-dependency fixture~~ -- done, see row 6 (`three-d`).
3. ~~`cargo audit` and `cargo deny check`~~ -- done, see row 9 above.
4. ~~CI: 3-platform matrix~~ -- configured and verified by real
   `origin/main` Actions runs, see row 8 above.
5. ~~Pairwise feature combination testing~~ -- done, see row 5 (all 6
   pairs pass, purely additive, no interaction issues).
6. **New, before rows 4/5 above can be marked fully met** (explicitly
   requested as a follow-up phase, sequenced after the guard study, not
   interleaved with it):
   - ~~Dedicated Miri regression test for UNSAFE-001~~ (array reference
     conversion) -- done, `tests/soundness/array_conversions.rs`, see
     row 4 above.
   - ~~Before/after benchmark for UNSAFE-003~~ -- done and adopted:
     `det_sub_proc_unsafe` was replaced with bounds-checked indexing,
     confirmed via release-build disassembly to compile to byte-identical
     machine code (zero cost), so the unchecked version was kept only in
     git history. UNSAFE-003 is resolved and removed from the unsafe
     inventory, see `docs/unsafe-audit.md`.
   - ~~UNSAFE-004 disposition~~ -- done: `src/quaternion_simd.rs` and
     `src/vector_simd.rs` (the `mem::uninitialized`-based pattern) are
     deleted, not just left flagged, since they were private, permanently
     unreachable from any declared Cargo feature. Zero public API diff
     confirmed before/after. See `docs/unsafe-audit.md`.
   - ~~`serde`: byte-for-byte serialized representation equality and
     round-trip, for representative types, against real `cgmath` 0.18.0~~
     -- done: `compat/fixtures/dual-dep/` extended with 6 differential
     tests (`Vector1..4`, `Point1..3`, `Matrix2..4`, `Quaternion`,
     `Euler<Rad/Deg<S>>`, `Decomposed`, `f32`+`f64` where applicable),
     each checking byte-identical JSON, same-crate round-trip, and
     cross-crate deserialization -- not just "does `cargo test --features
     serde` pass." All pass, byte-identical. Feature-off leak also
     confirmed: `cargo tree` (no flags, and `--no-default-features`) shows
     `serde` entirely absent from the dependency graph. See
     `docs/compatibility.md`'s differential-testing section.
   - ~~`mint`: component order, matrix orientation, and round-trip
     verification~~ -- done: `compat/fixtures/dual-dep/` extended with 5
     tests covering the full mint inventory (`Vector2..4`, `Point2..3`,
     `Matrix2..4`, `Quaternion`, `Euler`), every pair bidirectional. Uses
     pairwise-distinct component values so a swap/transpose/scalar-vector
     mixup would fail, not pass by coincidence; matrix orientation
     specifically checked against both the correct column (`m[i]`, must
     match) and the row (`m.row(i)`, must NOT match, since the test
     matrix is deliberately asymmetric). Feature-off leak confirmed via
     `cargo tree` the same way as `serde`. All pass, both against real
     `cgmath` 0.18.0 and `cgmath-next`.
   - ~~`rand`: confirm the public `Distribution` impls and feature
     isolation~~ -- done: every `Distribution` impl body
     (`Vector1..4`/`Matrix2..4`/`Quaternion`/`Rad`/`Deg`/`Euler`, 11
     concrete instantiations across 7 macro/hand-written sites) diffed
     byte-for-byte against the pristine 0.18.0 import -- identical, not
     just "same set of types". New `tests/rand_distribution.rs` (6
     tests, gated `#![cfg(feature = "rand")]`) confirms the existing
     contract these impls have always had: generated components are
     finite and within their documented/derivable range (`[0,1)` for
     vector/matrix/quaternion components, confirmed against rand 0.8.7's
     own source -- `distributions::Standard`'s docs and its
     `Distribution<f32/f64>` impl comment both state `[0, 1)` --
     `[-pi,pi)`/`[-180,180)` for `Rad`/`Deg` per the
     macro's own `gen_range` bounds), plus a non-degeneracy sanity
     check. Deliberately does NOT assert an exact RNG output sequence,
     since upstream never guaranteed one. Feature-off leak confirmed via
     `cargo tree` the same way as `serde`/`mint`.
   - ~~`swizzle`: API diff specifically with vs. without the feature
     enabled~~ -- done: rustdoc-JSON inventory of every swizzle method
     (identified by its generated doc comment) shows 550 methods with
     the feature on and 0 with it off, in both `cgmath-next` and
     pristine `cgmath` 0.18.0, zero difference either direction. Plus a
     compile-fail fixture (`compat/fixtures/swizzle-off/`) that
     concretely fails with `E0599: no method named `xy`` when the
     feature is off and builds cleanly when it's on. See
     `docs/compatibility.md`.

   **All 6 items in this phase are now complete.** Post-completion
   re-verification (same scope as requested -- public API diff, full
   `cargo test`, `--all-features`, all 6 pairwise combinations, targeted
   Miri (`tests/soundness`, `tests/matrix`, filtered `--lib`), all 5
   reverse-dependency fixtures, blocking GitHub Actions jobs): all pass,
   0 regressions, 0 unexplained differences. CI run
   [31943457863](https://github.com/kent-tokyo/cgmath-next/actions/runs/31943457863)
   (the swizzle-verification commit) green on every blocking job.
7. Human review and explicit publish approval -- done for `alpha.1`
   (2026-08-16). Stable requires its own separate approval; see
   "Stable release track" below.

## Stable release track (0.18.1-alpha.1 -> 0.18.1)

Started 2026-08-16, per explicit user instruction. Goal: lock the
verification `alpha.1` already has into continuous CI, ensure release
artifact reproducibility, and reach a defensible stable state -- not new
feature work.

### Scope freeze (in effect until 0.18.1 stable ships)

**Forbidden:** new features, new public API, public item removal, trait
bound changes, serialization format changes, matrix/vector/quaternion
layout changes, numeric convention changes, default feature changes,
MSRV changes, unnecessary new dependencies, large refactors, broad
performance-motivated changes.

**Allowed:** soundness fixes, compatibility fixes, release-infrastructure
fixes, CI regression prevention, documentation factual corrections,
package metadata fixes, release-blocker bug fixes.

If any change outside the allowed list becomes necessary before stable
ships, that's the trigger to insert `0.18.1-rc.1` and stop-and-report
(see "RC decision" below) -- not to proceed straight to stable anyway.

### Phase 1: release infrastructure lock-down

- [x] `compat` CI job (`.github/workflows/ci.yml`) already ran
  `dual-dep` (serde/mint differential) and `swizzle-off` (negative +
  positive control) as blocking steps on every push (added pre-`alpha.1`,
  commit `6bf4582`).
- [x] Swizzle-off negative control strengthened: now checks for
  `E0599`, `no method`, and `xy` in build output (previously only the
  first and third), so a differently-worded compiler error can't be
  mistaken for the expected one. Commit `dce66da`.
- [x] serde/mint/rand feature-leak check added to the `compat` job,
  verified in **both directions** (absent with no features, present
  with `--all-features`) so the check can't pass vacuously if it were
  ever pointed at the wrong dependency name. Previously only a one-time
  manual `cargo tree` check recorded in `docs/compatibility.md`. Commit
  `dce66da`.
- [x] `Cargo.toml`: `publish = ["crates-io"]` added, preventing an
  accidental publish to the wrong registry. Commit `9c9df1f`.
- [x] `publish.yml`: `permissions: contents: read` (least privilege) and
  a `concurrency: group: crates-io-publish, cancel-in-progress: false`
  block added. Commit `9c9df1f`.
- [x] `UNSAFE-002` explicitly documented as an accepted permanent known
  limitation of the stable series, not an open item -- see row 2 of the
  stable table above. This update.

**Phase 1 is complete** (user confirmation, 2026-08-16, CI green on
`49d7dca`).

`publish.yml`'s publish-safety measures, considered complete as-is:
manual `workflow_dispatch`-only trigger, typed version-string
confirmation checked against `Cargo.toml`, an in-job `cargo publish
--dry-run` immediately before the real publish, least-privilege
`permissions: contents: read`, a `concurrency` group preventing
overlapping publish attempts, `Cargo.toml`'s `publish = ["crates-io"]`
restricting the target registry, `--no-verify`/`--allow-dirty` never
used, and explicit human approval required before any dispatch. A
GitHub Environment with a required-reviewer protection rule was
considered and explicitly decided against: `cgmath-next` is developed
and published by one person, so a required-reviewer gate would mean
self-approval -- extra friction without a meaningful safety gain. Not
planned as future work either; the list above is the actual, final
publish-safety design.

### RC decision

**`0.18.1-rc.1` is not a mandatory step.** If no significant code-level
change occurs during this stabilization phase, proceed directly from
`alpha.1` to `0.18.1` stable after the final preflight -- do not insert
an RC just for its own sake. This reflects that `0.18.1` is a narrowly
scoped maintenance release (existing-API compatibility, a known-UB fix,
zero public API diff, reverse-dependency-verified, Miri/feature-matrix
verified), not a new-feature release, and there are essentially no
external users yet for an RC to add meaningful additional coverage over.

**Only insert `0.18.1-rc.1` and stop-and-report if, before stable, any
of the following happen:**
- The soundness implementation (`UNSAFE-001`-`004` or their guards) is
  changed again.
- Public API or a trait bound is touched.
- Anything touching serialization format or matrix/vector/quaternion
  layout changes.
- A dependency or the MSRV changes.
- A real compatibility problem is reported against `alpha.1` by an
  external user.

**Confirmed as of 2026-08-16 (`49d7dca`): no RC needed yet.** `git diff
--stat v0.18.1-alpha.1..HEAD` shows zero changes under `src/` or
`build.rs` -- every commit since the alpha tag is CI, publish-workflow,
package metadata, README, issue templates, or the new `examples/` file.
None of the RC triggers above have fired.

### Scope freeze: in effect now

As of 2026-08-16 (`49d7dca`), this repository is in the scope freeze
defined above -- no new features, refactors, dependency changes, MSRV
changes, or public API changes until `0.18.1` stable ships (or an RC
trigger fires). Stop and report immediately if an urgent soundness or
compatibility problem is found; otherwise no further code changes are
expected before the stable-publish work below is explicitly requested.

### Timeline

Not a fixed one-week ritual: `alpha.1` was never publicized, so a longer
wait doesn't mechanically increase the odds of external reports arriving.
Target: **around 2026-08-20**, contingent on no major issue surfacing
against `alpha.1` in the meantime, **and** the user's own explicit
instruction to proceed with the stable-publish work below -- reaching
the date alone does not authorize it. Upstream/RustSec reply status is
explicitly **not** part of this gate (see the project memory recorded
2026-08-16 correcting an earlier, overly cautious version of this same
plan).

### Stable-publish work (do NOT start until explicitly instructed, on or after ~2026-08-20)

1. `Cargo.toml` version -> `0.18.1`.
2. Update `README.md`/`README_ja.md`/`README_zh.md`, `CHANGELOG.md`, and
   this checklist to remove alpha-specific language.
3. Keep `UNSAFE-002` documented as an accepted known limitation of
   stable (already done, see row 2 above -- do not weaken this framing).
4. Write stable release notes.
5. Final preflight, same rigor as `alpha.1`'s:
   - `cargo test`, `cargo test --all-features`, `cargo test --doc`
   - `cargo package --list` (check for unexpected contents/bloat)
   - `cargo publish --dry-run` and `cargo publish --dry-run --all-features`
   - Extract the actual generated `.crate` and, from inside it, re-run
     `cargo test`, `cargo test --all-features`, `cargo test --doc`
   - Confirm all existing blocking CI jobs are green
   - **Do NOT** re-run the full Miri suite, the full reverse-dependency
     fixture set, or the 59-benchmark suite unless `src/` actually
     changed -- none of that changed since `alpha.1`, so those stay at
     their `alpha.1`-verified results.
6. If the preflight fails, or `cargo package --list` shows unexpected
   contents, **stop and report** -- do not fix-and-continue or publish
   unilaterally.
7. **Even after a clean preflight: do not run `cargo publish`, create a
   tag, or create a GitHub Release.** Report the preflight results and
   the exact commit SHA that would be published, and wait for explicit
   separate approval before executing any of those three -- same
   stop-and-report boundary as `alpha.1`'s publish, not inherited from
   it.
