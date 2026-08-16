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
| 1 | 公開版0.18.0のprovenanceが記録されている | done -- `docs/provenance.md` |
| 2 | Apache-2.0 attributionが保持されている | done -- `LICENSE`, per-file copyright headers unmodified |
| 3 | upstreamテストが通る | done -- 256/256 original tests pass unmodified, `docs/baseline.md`; benchmark baseline (§16) also now recorded there, 59/59 pass |
| 4 | swap_columnsの既知UBが修正されている | done, and broader than the ticket -- see the fix commit, `docs/unsafe-audit.md` |
| 5 | Miri regression testが通る | done -- `tests/soundness/`, 22/22 pass under `cargo +nightly miri test --test soundness` |
| 6 | public API差分が生成されている | done -- `docs/api-inventory.md`, zero-diff result |
| 7 | feature matrixが通る | done -- `docs/compatibility.md`, 6/6 individual rows + all-features + all 6 pairwise combinations |
| 8 | migration fixtureが3件以上通る | done, and exceeds the stable gate too -- `compat/fixtures/reverse-deps/RESULTS.md`, 5 real reverse-dependency crates |
| 9 | unsafe inventoryが完成している | done -- `docs/unsafe-audit.md`, 4 pattern groups covering every remaining `unsafe` |
| 10 | 各unsafeに監査状態が記録されている | done -- same doc, per-group Status field |
| 11 | README、CHANGELOG、SECURITY.mdが存在する | done |
| 12 | crates.io publishはまだ行っていない | done (trivially true) |

**All 12 alpha conditions are met as of this checklist's last update.**
`0.18.1-alpha.1` could be tagged/published pending explicit human
approval (§21) -- this document does not constitute that approval.

## 0.18.1 stable

| # | Condition | Status |
|---|---|---|
| 1 | 未説明のpublic API削除がゼロ | met -- zero API diff |
| 2 | 既知のsafe-to-UB経路がゼロ | **guarded, not literally zero** -- `UNSAFE-002` (tuple-layout transmute) now runs a runtime layout check (size/align/per-field offset) before every transmute in this category, panicking instead of transmuting on a mismatch; verified via Miri, a negative-control test, and release-build disassembly confirming zero cost when layout matches (see `docs/unsafe-audit.md`'s feasibility-study section). This converts the failure mode from silent UB to a loud panic -- it is **not** a language-level soundness proof, tuple layout remains officially unspecified, so this row is "guarded" rather than truly "met" |
| 3 | 残存unsafeのinvariantが文書化されている | met -- `docs/unsafe-audit.md` |
| 4 | release対象unsafe testがMiriで通る | in progress -- `UNSAFE-001` now has a dedicated Miri regression suite (`tests/soundness/array_conversions.rs`, 14 tests, all `AsRef`/`AsMut`/`From<&[..]>`/`From<&mut [..]>` paths across `Vector1-4`/`Point1-3`/`Matrix2-4`/`Quaternion`, including write-back-through-the-view checks and `-Zmiri-strict-provenance`); `UNSAFE-003`/`004` still pending |
| 5 | serde、mint、rand、swizzleの互換性が検証されている | mostly -- individual-feature and all 6 pairwise-combination `cargo test` runs pass for all 4 (`docs/compatibility.md`); no dedicated round-trip/format-stability test beyond what upstream's own tests already cover |
| 6 | migration fixtureが5件以上通る | met -- 5/5 (`arcball`, `crevice`, `truck-base`, `vector-traits`, `three-d`) |
| 7 | MSRVが実測・文書化されている | met -- `docs/msrv.md`, though it documents that the number is driven by transitive deps and will drift |
| 8 | 3 platformでCIが通る | met -- verified by real runs against `origin/main`, and it's caught 2 real issues so far (not just "ran green"): [run](https://github.com/kent-tokyo/cgmath-next/actions/runs/31937609325) 1 found a `cargo audit` lockfile bug (fixed, `825d7f5`); the [layout-guard commit's run](https://github.com/kent-tokyo/cgmath-next/actions/runs/31940302025) found the `miri` job's `--lib` sweep hitting Miri's own known float non-determinism in `slerp` for the first time (fixed by narrowing that step's scope, not by weakening any test, `f155eaf`; [confirmed green](https://github.com/kent-tokyo/cgmath-next/actions/runs/31940574715) after). Otherwise green across all blocking jobs except the pre-documented informational `fmt` job |
| 9 | `cargo audit`と`cargo deny`の結果が説明されている | met -- `deny.toml` added, `cargo deny --all-features check` run clean (advisories/bans/licenses/sources all ok); `security` CI job wires this in going forward |
| 10 | 既知のcompatibility gapが公開されている | met -- `docs/compatibility.md`, `docs/unsafe-audit.md` both list open gaps |
| 11 | release checklistが完了している | this document; not all rows above are checked yet |
| 12 | publish前に人間の明示承認を得ている | pending -- not requested yet |

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
   - Dedicated Miri regression test for UNSAFE-001 (array reference
     conversion), matching the coverage `tests/soundness/` already gives
     the swap_columns/swap_elements fix.
   - Before/after benchmark for UNSAFE-003 if its unchecked indexing
     were replaced with safe indexing (currently audited sound as-is,
     not blocking, but the comparison hasn't been measured).
   - `serde`: byte-for-byte serialized representation equality and
     round-trip, for representative types, against real `cgmath` 0.18.0
     -- not just "does `cargo test --features serde` pass."
   - `mint`: component order, matrix orientation, and round-trip
     verification (same "does it actually match", not just "does it
     compile" bar).
   - `rand`: confirm the public `Distribution` impls and feature
     isolation (no accidental behavior change when `rand` is off).
   - `swizzle`: API diff specifically with vs. without the feature
     enabled (beyond the pairwise test-count check already done).
7. Human review and explicit publish approval.
