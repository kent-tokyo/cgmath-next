# Release checklist

Tracks AGENTS.md §20's completion conditions. Update this file as items
change status -- it's the single source of truth for "are we ready to cut
a release," not a snapshot to be trusted stale.

**Neither series has been published to crates.io.** No tag exists. No
GitHub release exists. `crates.io` publish, tag creation, and default-
branch pushes are all AGENTS.md §21 stop-and-report items, not something
this project's automation does on its own.

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
| 2 | Zero known safe-to-UB paths | **guarded, not literally zero** -- `UNSAFE-002` (tuple-layout transmute) now runs a runtime layout check (size/align/per-field offset) before every transmute in this category, panicking instead of transmuting on a mismatch; verified via Miri, a negative-control test, and release-build disassembly confirming zero cost when layout matches (see `docs/unsafe-audit.md`'s feasibility-study section). This converts the failure mode from silent UB to a loud panic -- it is **not** a language-level soundness proof, tuple layout remains officially unspecified, so this row is "guarded" rather than truly "met" |
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
7. Human review and explicit publish approval.
