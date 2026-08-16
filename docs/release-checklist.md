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
| 7 | feature matrixが通る | done -- `docs/compatibility.md`, 6/6 individual rows + all-features, not the full pairwise combination matrix (noted as a gap there) |
| 8 | migration fixtureが3件以上通る | done -- `compat/fixtures/reverse-deps/RESULTS.md`, 3 real reverse-dependency crates |
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
| 2 | 既知のsafe-to-UB経路がゼロ | **not met** -- `UNSAFE-002` (tuple-layout transmute) remains unverified; `-Zrandomize-layout` was attempted and confirmed not to cover this case; independently corroborated as an unresolved upstream issue since 2021 (`rustgd/cgmath#538`), whose own collaborator concluded the only real fix is removing the reference-returning impls -- a public API change, out of scope here (see `docs/unsafe-audit.md`) |
| 3 | 残存unsafeのinvariantが文書化されている | met -- `docs/unsafe-audit.md` |
| 4 | release対象unsafe testがMiriで通る | met for what exists (`tests/soundness/`); `UNSAFE-001`/`003`/`004` don't have dedicated Miri regression tests, only the audit's evidence |
| 5 | serde、mint、rand、swizzleの互換性が検証されている | partially -- individual-feature `cargo test` passes for all 4 (`docs/compatibility.md`); no dedicated round-trip/format-stability test beyond what upstream's own tests already cover |
| 6 | migration fixtureが5件以上通る | **not met** -- 3/5 (`arcball`, `crevice`, `truck-base`) |
| 7 | MSRVが実測・文書化されている | met -- `docs/msrv.md`, though it documents that the number is driven by transitive deps and will drift |
| 8 | 3 platformでCIが通る | met on paper -- `.github/workflows/ci.yml` configures the 3-platform x {MSRV, stable, beta-nonblocking} matrix; **not yet verified by an actual GitHub Actions run** (never pushed to `origin`, see Parked items) |
| 9 | `cargo audit`と`cargo deny`の結果が説明されている | met -- `deny.toml` added, `cargo deny --all-features check` run clean (advisories/bans/licenses/sources all ok); `security` CI job wires this in going forward |
| 10 | 既知のcompatibility gapが公開されている | met -- `docs/compatibility.md`, `docs/unsafe-audit.md` both list open gaps |
| 11 | release checklistが完了している | this document; not all rows above are checked yet |
| 12 | publish前に人間の明示承認を得ている | pending -- not requested yet |

## Outstanding work before stable, in priority order

1. Resolve or formally accept `UNSAFE-002`'s risk (either rewrite the
   tuple transmutes as safe field-by-field code, or get a stronger
   verification than `-Zrandomize-layout` provides).
2. 2 more reverse-dependency fixtures (5 total). Candidates from the
   filtered `^0.18`/`^0.18.0` normal-kind reverse-dependency list not yet
   used: `three-d` (popular rendering engine, larger build), `vector-
   traits` (trait abstraction crate, likely light), `boostvoronoi_core`
   (geometry/CAD), `smithay` (Wayland compositor, heavier build).
3. ~~`cargo audit` and `cargo deny check`~~ -- done, see row 9 above.
4. ~~CI: 3-platform matrix~~ -- configured in `.github/workflows/ci.yml`;
   still needs a real run against `origin` to count as *verified*, not
   just configured (blocked on the parked push decision).
5. Pairwise feature combination testing beyond `--all-features`.
6. Human review and explicit publish approval.
