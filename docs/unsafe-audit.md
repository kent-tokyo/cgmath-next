# Unsafe audit

Every `unsafe` in the crate as of this commit, grepped exhaustively
(`unsafe fn`, `unsafe {`, `unsafe impl` -- none of the last found).
`swap_columns`/`swap_elements`/`Array::swap_elements` (RUSTSEC-2026-0197 and
its two sibling call sites) are not listed here: they were fully replaced
with safe code and are covered in the fix commit and
`tests/soundness/`, not carried forward as audited-but-kept unsafe.

Grouped by pattern rather than one row per macro-expanded call site, since
several entries below are a single hand-written pattern instantiated by a
macro across 7-9 types. Each group lists every source line it covers.

## Summary

| ID | Pattern | Reachable in default/normal builds? | Can safe Rust replace it? | Status |
|---|---|---|---|---|
| UNSAFE-001 | repr(C) struct <-> fixed-size array transmute | yes | not without changing the public `From`/`AsRef`/`AsMut` API shape | audited sound |
| UNSAFE-002 | repr(C) struct <-> homogeneous tuple transmute | yes | yes, at the cost of extra field-by-field code per arity | **flagged -- still unverified.** Confirmed empirically that `-Zrandomize-layout` does not randomize tuple layout at all (only structs/enums), so that tool cannot stress-test this entry either way (see detail below) |
| UNSAFE-003 | `det_sub_proc_unsafe` (unchecked indexing) | yes, for `Matrix4::determinant` (unconditionally); no, for `Matrix4::invert`'s SIMD branch | yes, with a bounds-checked rewrite; likely no measurable cost given the caller already holds a `&Matrix4` | audited sound (see caveats) |
| UNSAFE-004 | `mem::uninitialized` + external `simd` crate SIMD load/store | **no -- dead code** | irrelevant while dead; would need a full rewrite before ever enabling | dead code, do not enable as-is |

---

### UNSAFE-001: repr(C) struct <-> fixed-size array transmute

**Files/Functions:**
- `src/macros.rs:181,188,203,210` (`impl_fixed_array_conversions!` macro) --
  applied to `Vector1/2/3/4` (`src/vector.rs:379-382`) and `Point1/2/3`
  (`src/point.rs:358-360`)
- `src/matrix.rs:1474,1481,1496,1503` (`fixed_array_conversions!` macro,
  `[[S; n]; n]` form) and `src/matrix.rs:1518,1525,1540,1547` (flat
  `[S; n*n]` form) -- applied to `Matrix2/3/4`
- `src/quaternion.rs:553,560,574,581` (hand-written, `[S; 4]` form)

**Purpose:** implement `AsRef`/`AsMut`/`From<&_>`/`From<&mut _>` between
each type and its natural fixed-size-array representation, for zero-copy
interop (e.g. passing a `&Matrix4<f32>` to a graphics API expecting
`&[f32; 16]`).

**Safety invariant:** the source and target types must have identical size,
alignment, and field layout, so that reinterpreting the reference's
pointee via `mem::transmute` (or, in the fixed-array-conversions macro
case, one already-transmuted step reused by `Index`) is not UB.

**Caller requirements:** none beyond normal borrow rules -- these are safe
public functions; the invariant is a property of the type definitions, not
something a caller must uphold.

**Can safe Rust replace it?** Not without changing the public API to return
owned arrays (`into()`, already available) instead of `&T`/`&mut T` views.
`AsRef`/`AsMut`/borrowed `From` are part of 0.18.0's API surface and
AGENTS.md treats API removal as a stop-and-report item, so this session
keeps the pattern.

**Why it's sound:** every source type (`Vector1/2/3/4`, `Point1/2/3`,
`Matrix2/3/4`, `Quaternion`) is `#[repr(C)]` (verified: every struct
definition carries an explicit `#[repr(C)]`, see e.g. `src/vector.rs`
`Vector4`, `src/point.rs` `Point3`, `src/matrix.rs` `Matrix2/3/4`,
`src/quaternion.rs` `Quaternion`) with N fields of the same element type `S`
and no padding (`S` is always a `BaseNum`, i.e. a primitive numeric type
whose size equals its alignment). `#[repr(C)]` with homogeneous same-size,
same-align fields and no padding produces a layout byte-identical to
`[S; N]` (or `[[S; n]; n]` for matrices, column-then-row). This is the
standard, widely-relied-upon pattern for this category of crate (bytemuck's
`Pod` derive works on exactly this class of type for the same reason).

**Miri coverage:** not directly targeted. Exercised incidentally by
existing conversion tests (`tests/vector.rs`, `tests/matrix.rs`,
`tests/point.rs` `From`/`Into` array round-trips) but those were not run
under Miri in this session -- only the swap regression suite
(`tests/soundness`) and `tests/matrix.rs` were (see below).

**Tests:** `tests/vector.rs`, `tests/point.rs`, `tests/matrix.rs` (array
round-trip assertions, pre-existing upstream tests, unmodified).

**Status:** audited sound. Not re-verified under Miri with
`-Zmiri-strict-provenance` in this session -- recommended before the
0.18.1 stable gate.

---

### UNSAFE-002: repr(C) struct <-> homogeneous tuple transmute

**Files/Functions:**
- `src/macros.rs:229,236,250,257` (`impl_tuple_conversions!` macro) --
  applied to `Vector1/2/3/4` (`src/vector.rs:384-387`) and `Point1/2/3`
  (`src/point.rs:362-364`)
- `src/quaternion.rs:600,607,623,630` (hand-written, `(S, S, S, S)` form)

**Purpose:** same as UNSAFE-001, but targeting Rust tuples
(`(S, S)`, `(S, S, S)`, `(S, S, S, S)`) instead of arrays, for call sites
that prefer tuple destructuring.

**Safety invariant:** same as UNSAFE-001 -- source and target must be
layout-identical.

**Caller requirements:** none (safe public API).

**Can safe Rust replace it?** Yes: `AsRef`/`AsMut` returning `&Tuple` could
be replaced by constructing an owned tuple via `Into` and dropping the
borrowed forms, but that is a public API removal (AGENTS.md stop-and-report
territory) so not done this session.

**Why this one is flagged, unlike UNSAFE-001:** a plain Rust tuple
`(S, S, S, S)` is **not** `#[repr(C)]` and the Rust reference explicitly
states tuple field layout is unspecified -- the compiler is free to reorder
tuple fields in principle. In practice, for a homogeneous tuple of
identically-sized, identically-aligned primitive fields, current rustc has
no incentive to reorder and empirically lays them out in declaration order
(this code has shipped this way since early cgmath versions without a
known miscompilation). But "empirically stable" is a materially weaker
guarantee than UNSAFE-001's "guaranteed by `#[repr(C)]`", and this is
exactly the kind of thing `-Zrandomize-layout` (a Miri/rustc flag that
deliberately randomizes non-`repr(C)` layout to catch code relying on
accidental field order) is designed to catch.

**Miri coverage (updated after initial audit):**
`RUSTFLAGS="-Zrandomize-layout" cargo +nightly miri test --lib` was run
against the crate's own `#[cfg(test)] mod tests` unit tests (found in
`src/vector.rs`, `src/point.rs`, `src/quaternion.rs` -- these exercise
`as_ref`/`as_mut`/`into`/`from` for both the array and tuple forms, e.g.
`quaternion::tests::test_into` checks `let v: (f32,f32,f32,f32) = v.into();
assert_eq!(v, (1.0, 2.0, 3.0, 4.0));`). Result: **every
`test_as_ref`/`test_as_mut`/`test_into`/`test_from` test passed** for
`Vector2/3/4`, `Point2/3`, and `Quaternion` (72 total lib tests run, 70
passed; the 2 failures were `quaternion::tests::test_slerp_extrapolate`/
`test_slerp_half`, both `assert_ulps_eq!` floating-point comparisons in
spherical interpolation -- unrelated to any conversion/transmute code path,
same category of Miri float non-determinism recorded in `docs/baseline.md`
for `rotate_from_euler::test_y`).

**`-Zrandomize-layout` does NOT cover tuples -- confirmed empirically, not
just passed-by-accident.** Before trusting the Miri result above, checked
directly with two throwaway single-file programs compiled with
`rustc +nightly -Zrandomize-layout -Zlayout-seed=N` for several `N`:

* A plain `#[repr(Rust)]` struct with 4 `f32` fields: field byte offsets
  actually change across seeds (e.g. seed 1 -> `4 0 12 8`, seed 2 ->
  `12 8 4 0`, seed 3 -> `0 12 8 4`) -- confirms the flag and this
  methodology both work as expected.
* The tuple `(f32, f32, f32, f32)`: byte offsets are `0 4 8 12` for
  **every** seed tested (1, 2, 3) -- i.e. always plain declaration order,
  never reordered.

So the earlier "70/72 conversion tests pass under `-Zrandomize-layout`"
result **does not actually stress-test UNSAFE-002's core risk**. It only
re-confirms that current rustc's tuple layout algorithm happens to use
declaration order (which was already known/assumed) -- `-Zrandomize-layout`
apparently only randomizes nominal (struct/enum) types, not tuples, so it
can't disprove or stress a tuple-layout assumption either way. This is a
more useful, if less reassuring, finding than "it passed": it means no
tool available in this session can mechanically stress-test UNSAFE-002,
and the risk remains exactly what it was in the first audit pass --
empirically stable, not language-guaranteed.

**Tests:** `src/vector.rs`, `src/point.rs`, `src/quaternion.rs`
`#[cfg(test)] mod tests` (pre-existing upstream unit tests, unmodified).

**Status:** still flagged, not fixed, and **not meaningfully re-verified**
despite the `-Zrandomize-layout` attempt -- see above for why that attempt
doesn't apply to tuples. The only mechanical way to close this out found so
far in this session's toolset is: replace the tuple transmutes with
explicit field-by-field construction (safe, no perf cliff expected at
`-O`), which is the same recommendation as the original audit, now with
more confidence that no cheaper verification shortcut exists.

---

### UNSAFE-003: `det_sub_proc_unsafe`

**File/Function:** `src/matrix.rs:1739` (`unsafe fn det_sub_proc_unsafe`),
called from `Matrix4::determinant` (`src/matrix.rs:865`, unconditional) and
`Matrix4::invert`'s `#[cfg(feature = "simd")]` branch
(`src/matrix.rs:922,930,931,932`).

**Purpose:** compute cross-product-like sub-terms of the Matrix4 adjugate
using `get_unchecked` on a `&[S; 16]` view of the matrix, avoiding bounds
checks in a function called on every `determinant()`/`invert()` call.

**Safety invariant:** `x`, `y`, `z` (and the derived offsets `x+4`, `x+8`,
`x+12`, etc., all `< 16`) must be valid indices into the 16-element array,
i.e. `x, y, z < 4`.

**Caller requirements:** callers must only pass `x, y, z` in `0..4`.

**Can safe Rust replace it?** Yes, trivially -- `s[4 + x]` instead of
`s.get_unchecked(4 + x)` -- since every call site passes a literal constant
(`1, 2, 3` / `0, 3, 2` / `0, 1, 3` / `0, 2, 1`), so the bounds check would
be a compile-time-constant, almost certainly optimized away at `-O`, or at
worst a handful of cheap comparisons on a function that also does 12+
floating-point multiplications. This was **not changed** in this session
(out of scope for the swap_columns fix, and AGENTS.md's performance policy
says don't touch unrelated hot paths without a measured before/after), but
is the clearest "safe Rust replace it" candidate in this audit for a future
Phase 3 pass.

**Miri coverage:** exercised indirectly -- `cargo +nightly miri test --test
matrix` was run in this session (see docs/baseline.md and the fix commit)
and every `determinant`/`invert` test passed under Miri with no UB
reported for this function. (One unrelated failure was observed in that
run, `matrix3::rotate_from_euler::test_y` -- a floating-point rounding
difference between Miri's interpreter and native trig evaluation, not a
memory-safety report; it doesn't touch this function or any matrix
constructed with `#[cfg(feature = "simd")]`.)

**Tests:** `tests/matrix.rs` `test_determinant`, `test_invert` (pre-existing
upstream tests, unmodified).

**Status:** audited sound for all current call sites (all use literal
in-range indices). All call sites within `invert`'s SIMD branch are dead
code (see UNSAFE-004 -- `simd` is not a resolvable feature), so in a normal
build only the `determinant()` call site is live.

---

### UNSAFE-004: `mem::uninitialized` + external `simd` crate load/store

**Files:** `src/quaternion_simd.rs:28` (`impl From<Simdf32x4> for
Quaternion<f32>`); `src/vector_simd.rs:30,246,326` (`impl From<Simdf32x4>
for Vector4<f32>`, `impl From<Simdi32x4> for Vector4<i32>`, `impl
From<Simdu32x4> for Vector4<u32>`).

**Purpose:** construct a `Quaternion<f32>`/`Vector4<{f32,i32,u32}>` from a
128-bit SIMD register by allocating uninitialized storage, then
immediately overwriting all of it via the SIMD `store` intrinsic.

**Safety invariant (as written):** every byte of `ret` must be written by
`f.store(...)` before any read of `ret` -- which holds here, `store` writes
all 4 elements unconditionally.

**Is this code reachable?** **No, in any standard build.** Both files are
gated `#[cfg(feature = "simd")]` (`src/lib.rs:103,108`). Confirmed in this
session's feature inventory (`docs/compatibility.md`) that `simd` is not a
resolvable Cargo feature at all in 0.18.0 -- there is no `[features] simd =
[...]` entry, and the `simd` optional dependency itself is commented out in
`Cargo.toml` (`#simd = { version = "0.2", optional = true }`, "disabled
indefinitely" per the adjacent comment). Enabling this code would require
both hand-editing `Cargo.toml` to re-add a `simd = "0.2"` dependency *and*
inventing a `simd` Cargo feature that doesn't exist upstream -- not
something a normal `cargo build --features ...` invocation can reach.

**Caller requirements:** N/A while unreachable.

**Can safe Rust replace it?** Irrelevant while dead. If this were ever
revived, `mem::uninitialized` is deprecated (since Rust 1.39, in favor of
`MaybeUninit`) and is documented to be unsound for most types the moment
it's called, even before any read -- for `f32`/`i32`/`u32` arrays
specifically it happens to be one of the few cases the old API could
arguably get away with (no validity invariant beyond bit pattern), but it
should still be rewritten with `MaybeUninit` rather than kept as-is if
revived.

**Miri coverage:** none -- cannot be reached without a manual
`Cargo.toml`/feature edit this session did not make.

**Tests:** none (unreachable).

**Status:** dead code, flagged rather than fixed or deleted this session.
AGENTS.md's initial scope excludes "SIMDの全面再設計" (SIMD redesign), and
this is the whole reason that exclusion exists -- the `simd` crate
integration was already "disabled indefinitely" by upstream before 0.18.0
shipped. Recommend for a future phase: either delete
`quaternion_simd.rs`/`vector_simd.rs` and the dead `#[cfg(feature =
"simd")]` gates entirely (they cannot currently be exercised, tested, or
even compiled without external changes), or, if SIMD support is wanted
later, rewrite from scratch against a maintained SIMD crate with
`MaybeUninit`. Not deleting in this session because removing source files
wholesale is a bigger decision than this checkpoint's scope, and doing so
quietly would read as more invasive than what was asked for.

---

## `#![deny(unsafe_op_in_unsafe_fn)]`

Not added to the crate root in this session. AGENTS.md section 10 asks for
this "once MSRV allows" and to add it incrementally if the audit isn't
complete -- given `det_sub_proc_unsafe` (UNSAFE-003) is the only remaining
`unsafe fn` in the crate, and its body's unsafe operations are already
wrapped in the outer `unsafe fn`'s implicit unsafe scope (pre-2024-edition
behavior, matching this crate's unspecified/2015 edition), adding the lint
now would require either an edition bump (out of scope, MSRV not yet
measured -- see the "next recommended work" section of the final report)
or wrapping the 12 `get_unchecked` calls in their own inner `unsafe {}`
blocks. Deferred to the MSRV-measurement phase so the two changes land
together and can be evaluated as one unit.

## SAFETY comments

None of the four groups above have inline `// SAFETY:` comments in the
source yet (upstream 0.18.0 never had them either, aside from the
now-deleted sarcastic one in `Array::swap_elements`). AGENTS.md requires
specific, non-generic `SAFETY:` comments on every *remaining* unsafe block.
This session prioritized removing the swap-family unsafe over annotating
what's left; adding `SAFETY:` comments to UNSAFE-001 through UNSAFE-004
(content drawn directly from the "Safety invariant" fields above) is
listed as next recommended work.
