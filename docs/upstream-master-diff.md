# Upstream `master` diff (as of 2026-08-16)

`git log --oneline v0.18.0..master` against `https://github.com/rustgd/cgmath`
lists 24 commits (12 non-merge). None are imported into `cgmath-next` yet — this
is a classification only, per AGENTS.md Phase 0.

## New features (out of initial scope, not imported)

* `a01a6cb` `8c29a7e` `65aac05` `0637f57` — Planar (perspective/orthographic
  hybrid) projection, new `PlanarFOV` type in `src/projection.rs`. New public
  API surface — would be a compatibility-breaking addition to our v0.18.0
  baseline API inventory if pulled in now. Deferred.
* `11a5346` — `bytemuck` cast support (`Pod`/`Zeroable` impls gated behind a
  new optional dependency). New public API + new dependency. Deferred.
* `d4d19b9` — `IntoMint` trait implementations for Vector/Point/Matrix/
  Quaternion, plus mint bumped to 0.5.8. New public API. Deferred.
* `ada4add` `81583b2` — Convert several `Into` impls to `From` impls. This
  **changes public trait implementations** (adds `From`, and per Rust's
  blanket `impl<T, U: From<T>> Into<U> for T`, the reverse `Into` keeps
  working, so it is source-compatible) but is still an API-surface change
  relative to the pinned 0.18.0 baseline. Deferred — evaluate at Phase 4
  under "trait impl addition" rules if desired later.

## Refactors with no intended behavior change

* `af12763` `41fb64c` — Replace `BaseFloat`/`Float` trait bounds with
  `BaseNum`/`Num` where possible; remove `NumCast` bound from `BaseNum`. This
  **loosens trait bounds** on some generic functions. Loosening (not
  tightening) a bound is source-compatible for callers. Not imported in this
  session; flagged for later API-diff review since AGENTS.md treats "trait
  bound強化" (strengthening) as disallowed but is silent on loosening.
* `33fb2fd` `2e76e82` `df21854` `575c458` `fb205c0` — clippy/rustfmt cleanup
  commits. Pure style, no behavior change per their diffs. Not imported —
  baseline clippy/fmt state is tracked separately (see
  `baseline-results/06-fmt-check.log`, `baseline-results/07-clippy.log`) and
  will be brought in through `cgmath-next`'s own incremental clippy policy
  (AGENTS.md §15), not by cherry-picking upstream's.

## Dependency version bump

* `6c7c68f` — `approx` `0.4` → `0.5`. This is a **public-API-affecting
  dependency update** (approx's traits appear in cgmath's public API via
  `AbsDiffEq`/`RelativeEq`/`UlpsEq`). Per AGENTS.md §13 this needs explicit
  classification before adoption; deferred to Phase 6.

## Documentation only

* `e57b543` — fix a broken/possibly-malicious external link in
  `quaternion.rs` doc comments. Worth cherry-picking later as a
  documentation-only change (no code diff), but out of scope for this session
  since it touches doc comment text and the phase 1 rule is "no unrelated
  content changes" — flagged, not applied.

## Soundness

**No commit in this range touches `swap_columns`, `swap_rows`, `swap_elements`,
or the `ptr::swap` usage in `src/matrix.rs`.** Confirmed by grep against
`master`'s `src/matrix.rs`: the exact same
`unsafe { ptr::swap(&mut self[a], &mut self[b]) }` pattern from 0.18.0 is
still present, unfixed, on master. RUSTSEC-2026-0197's advisory also lists
`patched = []` (no patched version exists anywhere upstream). The fix in this
repository (Phase 2) is therefore original work.

## Conclusion for this session

Nothing from `master` was imported. All 24 commits are either new public API
(deferred as out of initial scope per AGENTS.md §6), a dependency bump
requiring its own classification (§13), or style-only cleanup superseded by
this repo's own clippy/fmt policy. This keeps the "faithful 0.18.0 baseline"
commit boundary clean.
