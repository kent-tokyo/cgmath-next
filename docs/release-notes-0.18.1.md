# cgmath-next 0.18.1 release notes

## Positioning

`cgmath-next` is an independently maintained, community successor to
`cgmath` 0.18.0. This is not a new-feature fork: the goal is soundness
and continued maintenance on top of the existing `cgmath` 0.18 API, not
a redesign. The compiled library crate name is still `cgmath`, so
`use cgmath::...` keeps working unchanged for the large majority of
existing `cgmath` users.

## Migration

```toml
[dependencies]
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

No source changes needed for the vast majority of users.

## Soundness fixes

`Matrix{2,3,4}::swap_columns` and `Matrix{2,3,4}::swap_elements` (and,
by extension, every path that calls them, including `transpose_self` --
`Matrix2/3/4::transpose_self` calls `Matrix::swap_elements` directly)
could reach undefined behavior from 100% safe Rust
([RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html),
[rustgd/cgmath#565](https://github.com/rustgd/cgmath/issues/565), still
open upstream). The unsound pattern was two sequential `IndexMut`
reborrows of the same matrix via `unsafe { ptr::swap(...) }`.

Fixed by replacing that pattern with a safe read-into-temporary-then-
write sequence. The fix is broader than the advisory's literal wording:

- The advisory names only *same-index* `swap_columns` calls. This
  project found and fixed the same bug for `swap_columns` with *any*
  two indices, not just equal ones -- two sequential `IndexMut`
  reborrows of the same matrix are unsound regardless of whether the
  indices happen to match.
- Two more call sites the advisory doesn't name were fixed the same
  way: `Array::swap_elements` (shared by `Vector2/3/4`, `Point2/3/4`,
  and `Matrix::swap_rows`), and `Matrix`'s own `(col, row)`
  `swap_elements`.

See `docs/unsafe-audit.md` and the commit history for full detail.

## Verification

- **Public API**: zero unexplained differences against pristine
  `cgmath` 0.18.0, machine-verified (`docs/api-inventory.md`).
- **Upstream test compatibility**: 256/256 original `cgmath` 0.18.0
  tests pass unmodified (`docs/baseline.md`).
- **Miri**: a targeted regression suite covers every soundness-relevant
  path -- the original `swap_columns`/`swap_elements` fix
  (`tests/soundness/`), and a dedicated suite for array-reference
  conversions (`tests/soundness/array_conversions.rs`, 14 tests across
  `Vector1-4`/`Point1-3`/`Matrix2-4`/`Quaternion`, including write-back-
  through-the-view checks under `-Zmiri-strict-provenance`).
- **serde/mint/rand/swizzle compatibility**: verified via differential
  testing against real `cgmath` 0.18.0, not just "tests pass" --
  byte-exact serde JSON round-trips, mint component-order and matrix-
  orientation checks (pairwise-distinct test values to positively catch
  a swap/transpose, not just avoid failing on symmetric data), rand
  `Distribution` impls confirmed byte-identical to pristine 0.18.0
  source, and a machine-verified 550-method swizzle inventory plus a
  real compile-fail fixture proving the feature gate holds. See
  `docs/compatibility.md`.
- **Reverse-dependency fixtures**: 5 real downstream crates (`arcball`,
  `crevice`, `truck-base`, `vector-traits`, `three-d`) migrate cleanly
  (`compat/fixtures/reverse-deps/RESULTS.md`).
- **CI**: Linux, Windows, and macOS, `cargo audit`/`cargo deny`, and a
  continuous `compat` job (not just point-in-time) covering the serde/
  mint/swizzle/feature-leak checks above.

## Known limitation

One category of unsafe code (`AsRef`/`AsMut`/`From<&Tuple>`/
`From<&mut Tuple>` conversions between `Vector1..4`/`Point1..3`/
`Quaternion` and their homogeneous-tuple forms, tracked as `UNSAFE-002`)
relies on tuple memory layout that the Rust language reference does not
formally guarantee.

This is **guarded**, not eliminated: a runtime check verifies size,
alignment, and per-field byte offsets before every such conversion, and
**panics instead of transmuting** if they don't match. Verified via
Miri, a negative-control test, and release-build disassembly confirming
zero cost when layout matches, as it does on every platform tested. This
converts a hypothetical future layout divergence from silent undefined
behavior into a loud, immediate panic.

This is **not** a language-level soundness proof -- tuple layout remains
officially unspecified by Rust. Closing it fully would mean removing the
reference-returning conversions, a public API change that would break
source compatibility with `cgmath` 0.18.0; this project's compatibility
policy keeps that API surface for the `0.18.x` series specifically to
preserve source compatibility, and accepts the runtime guard as the
tradeoff. `0.18.1` accepts this as a permanent known limitation of the
stable series, not an item still pending resolution.

**We do not claim "all unsafe code has been removed" or "soundness has
been fully proven."** What's claimed, precisely: the specific known
UB reachable from safe Rust (RUSTSEC-2026-0197 and its broader scope,
above) is fixed; every other unsafe pattern in the crate is inventoried,
audited, and either resolved, deleted, or -- in this one case --
converted from a silent to a loud failure mode. See
`docs/unsafe-audit.md` for the complete, per-pattern accounting.
