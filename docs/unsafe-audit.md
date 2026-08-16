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
| UNSAFE-002 | repr(C) struct <-> homogeneous tuple transmute | yes | only the owned `Into<Tuple>` impl (already safe); `AsRef`/`AsMut`/`From<&_>` need a public API removal, see detail below | **guarded and audited.** A runtime layout check (`tuple_layout_matches!`) now runs before every transmute in this category, panicking instead of transmuting on a layout mismatch -- verified via Miri, disassembly (confirmed zero-cost when layout matches), and a negative-control test. Not a language-level soundness proof -- see the feasibility-study section below for exactly what is and isn't established |
| UNSAFE-003 | `det_sub_proc` (was `det_sub_proc_unsafe`, unchecked indexing) | n/a -- no longer `unsafe` | **done.** Replaced with bounds-checked indexing; release-build disassembly confirmed byte-identical machine code to the old unchecked version, i.e. zero cost, not just "likely" | **resolved -- removed from the unsafe inventory** |
| UNSAFE-004 | `mem::uninitialized` + external `simd` crate SIMD load/store | **no -- was dead code** | n/a | **resolved -- deleted.** `src/quaternion_simd.rs` and `src/vector_simd.rs` (the only files containing this pattern) are removed, not just left disabled. See detail below |

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

**Miri coverage:** dedicated regression suite added,
`tests/soundness/array_conversions.rs` (wired into the existing
`tests/soundness.rs` harness via `#[path]`, same pattern as
`swap_columns.rs`/`swap_elements.rs`). Covers all 8 types in this group
(`Vector1/2/3/4`, `Point1/2/3`, `Quaternion`, plus both the nested
`[[S; n]; n]` and flat `[S; n*n]` array forms for `Matrix2/3/4`) across
all 4 directions (`AsRef`, `AsMut`, `From<&[..]>`, `From<&mut [..]>`).
Each test checks not just values but write-back through the transmuted
reference in both directions: mutating the array view returned by
`as_mut()` is asserted to change the original struct's fields, and
mutating the struct view returned by `From<&mut [..]>` is asserted to
change the original array -- the case Miri's aliasing model can actually
catch that a value-only assertion cannot. 14/14 pass under `cargo
+nightly miri test --test soundness array_conversions`, and again
identically under `MIRIFLAGS="-Zmiri-strict-provenance"`. The full
`tests/soundness` binary (36 tests: these 14 plus the pre-existing
swap-family regressions) also passes complete under both plain `cargo
test --test soundness` and unfiltered `cargo +nightly miri test --test
soundness` -- fast enough that CI's existing unfiltered
`cargo miri test --test soundness` step already exercises this suite on
every push with no workflow change needed.

**Tests:** `tests/vector.rs`, `tests/point.rs`, `tests/matrix.rs` (array
round-trip assertions, pre-existing upstream tests, unmodified);
`tests/soundness/array_conversions.rs` (new, this session, Miri-targeted).

**Status:** audited sound, and now with dedicated Miri regression
coverage including `-Zmiri-strict-provenance` (previously recommended
as outstanding before the stable gate; now done).

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

**External corroboration:** this is not a novel finding of this session --
upstream tracks the identical bug at
[`rustgd/cgmath#538`](https://github.com/rustgd/cgmath/issues/538)
("Unspecified behavior in `impl_tuple_conversions`"), open since
2021-08-18, still open as of this writing. The issue quotes the exact same
`AsRef`/`AsMut` transmute and the same Rust reference passage ("Tuples do
not have any guarantees about their layout"). A commenter
([`Mokuzzai`](https://github.com/rustgd/cgmath/issues/538#issuecomment-902028519))
independently confirmed the bug spans the same surface this audit lists --
`AsRef`/`AsMut`/`From<&_>`/`From<&mut _>` for every vector, point, and the
quaternion -- and proposed a compile-/runtime layout check as a mitigation
(not implemented upstream). A cgmath collaborator
([`aloucks`](https://github.com/rustgd/cgmath/issues/538#issuecomment-950041311))
concluded "those impls should be deprecated and then later removed in a
subsequent release" -- i.e. upstream's own maintainer-level assessment
independently agrees the only real fix is API removal, not a safe
rewrite. No fix has landed in the ~5 years since.

**Correction to the original audit's fix suggestion:** "replace with
field-by-field construction" only applies to the *owned* `Into<Tuple>`
impl, which is already safe (see the macro above -- it pattern-matches and
rebuilds by value, no transmute). It does **not** apply to
`AsRef`/`AsMut`/`From<&Tuple>`/`From<&mut Tuple>`: these return a
*reference* borrowed from `self`'s own memory, which cannot be produced
from freshly-constructed field values -- there is no safe-Rust way to
hand back `&(S, S, S, S)` that aliases `&self`'s bytes without either a
transmute or `self` already being stored as that tuple internally
(a representation change, not just a body rewrite). This matches
upstream's own conclusion: the only real fix is removing the
reference-returning impls, which is a public API change and therefore
AGENTS.md stop-and-report territory, not something to do unilaterally.

**Status (superseded by the layout-guard feasibility study below):** the
paragraph above described the state before a runtime layout guard was
implemented and verified. See the next section for the current status.

## UNSAFE-002 layout-guard feasibility study

A one-time feasibility study, explicitly scoped and approved as such (not
a general go-ahead to keep iterating on this indefinitely): can a runtime
check, evaluated before every tuple transmute, convert a hypothetical
future layout divergence from silent UB into a detected, loud failure --
without changing the public API, without raising the MSRV, and ideally at
zero cost when the layout actually matches (the case today, on every
platform this crate has ever been tested on)?

**Answer: yes, on all three counts, verified rather than assumed.**

### What the guard is, and precisely what it does and does not prove

Every one of the 32 conversion sites this section covers (8 types --
`Vector1/2/3/4`, `Point1/2/3`, `Quaternion` -- x 4 directions each --
`AsRef`, `AsMut`, `From<&Tuple>`, `From<&mut Tuple>`) now runs this check
immediately before its `mem::transmute` call:

```rust
assert!(
    tuple_layout_matches!(Vector4<S> { x: 0, y: 1, z: 2, w: 3 }, (S, S, S, S)),
    "cgmath-next: internal invariant violated -- ..."
);
```

`tuple_layout_matches!` (defined once in `src/macros.rs`, reused by every
macro-generated type; `Quaternion` has a hand-written equivalent,
`quaternion_tuple_layout_matches`, since its fields are nested through
`v: Vector3<S>` rather than flat) checks, at the point of the call:

1. `size_of::<Struct<S>>() == size_of::<Tuple>()`
2. `align_of::<Struct<S>>() == align_of::<Tuple>()`
3. every field's byte offset in `Struct<S>` equals the corresponding
   positional element's byte offset in `Tuple`

Offsets are computed via `addr_of!` on `MaybeUninit` storage and pointer
subtraction, not `core::mem::offset_of!` -- see the MSRV section below for
why. If any of the three checks fails, the code `panic!`s (via `assert!`)
instead of transmuting.

**This does NOT make the transmute language-guaranteed sound.** Plain
Rust tuple layout remains unspecified by the language reference; nothing
in this guard changes that fact, and nothing could without either
`#[repr(C)]` on tuples (not a thing Rust has) or removing the
reference-returning conversions entirely (the actual fix, per both this
audit's original conclusion and upstream's own `rustgd/cgmath#538`
collaborator, and still out of scope here as a public API change). What
the guard *does* provide: the specific failure mode this audit is
concerned with -- a future rustc, a future target, or a future
codegen-flag combination silently laying tuples out differently than this
crate's structs -- stops being silent memory corruption and becomes an
immediate, loud, deterministic panic, on the very first call after the
divergence, on this platform, in this binary. That is a real, meaningful
safety improvement over the unguarded transmute. It is a tripwire, not a
proof, and the entry below is worded to keep that distinction explicit
rather than imply more than was actually established.

### MSRV: three options compared, one required no tradeoff

`std::mem::offset_of!` (including tuple-index syntax, `offset_of!((S,S,S,S), 0)`)
was confirmed empirically, not assumed from documentation, to require
Rust 1.77: fails with `error[E0658]: use of unstable library feature` on
1.74.0, compiles and runs correctly on 1.77.0, both tested directly on
locally-installed toolchains. This crate's declared MSRV is 1.71
(`docs/msrv.md`), so building the guard on `offset_of!` would have forced
an MSRV bump.

A portable equivalent -- `addr_of!` on `MaybeUninit` storage plus pointer
subtraction, computing exactly the same byte offsets without the macro --
was built and cross-checked against `offset_of!`'s output on a 1.77+
toolchain (both agree exactly, for `f32`, `f64`, and `i32` component
types), then confirmed to compile and run correctly standalone on Rust
1.71.0 itself, with no unstable features. `MaybeUninit` (stable since
1.36) and `addr_of!` (stable since 1.51) both predate the declared MSRV
by years.

| Option | Outcome |
|---|---|
| (a) Internal MSRV-1.71-compatible helper | **Chosen and implemented.** No dependency, no MSRV change, verified to compile and pass the full test suite on both stable and 1.71.0 directly. |
| (b) Minimal dependency (e.g. `memoffset`) | Not needed -- (a) achieves the same thing with zero added dependencies once the portable pointer-arithmetic approach was confirmed to work, so this option has no advantage over (a) and wasn't pursued further. |
| (c) MSRV bump to 1.77 | Not needed, and correctly not done -- an MSRV change is an AGENTS.md stop-and-report item regardless of whether this study needed it, which it didn't. |

### Coverage

All 32 sites: `Vector1/2/3/4` and `Point1/2/3` via the shared
`impl_tuple_conversions!` macro (`src/macros.rs`, invoked from
`src/vector.rs`/`src/point.rs` with explicit tuple-index annotations added
to each call site), and `Quaternion` by hand (`src/quaternion.rs`, guarded
via `assert_quaternion_tuple_layout!`). Every `AsRef`/`AsMut`/
`From<&Tuple>`/`From<&mut Tuple>` impl in this category now runs the
check; the already-safe owned `Into<Tuple>`/`From<Tuple>` impls are
unaffected (they never transmuted anything).

### Negative control

`src/macros.rs`, `mod tuple_layout_guard_tests` (new, `#[cfg(test)]`,
3 tests): `accepts_the_real_correct_mapping` (positive control -- the
real `Vector4<f32>`/`Vector4<f64>` mapping used by the crate's own call
sites passes, or the guard would break every tuple conversion in the
crate), `rejects_a_scrambled_field_to_index_mapping` (the actual negative
control -- deliberately invokes `tuple_layout_matches!` with a reversed
field/index mapping against the *real* `Vector4<f32>` type and asserts it
returns `false`), and `rejects_a_size_mismatch` (same macro, a
differently-sized tuple type). All three call the boolean-returning
`tuple_layout_matches!` macro directly and never call `mem::transmute` --
`addr_of!` on `MaybeUninit` storage never reads the pointee or creates a
reference, so nothing in any of the three tests, including the two
deliberately-wrong ones, can be unsound.

### Verification performed

- `cargo test --all-features`: 306/306 pass (303 previously + 3 new guard
  tests), 0 failed, both on stable and on `cargo +1.71.0 test
  --all-features` directly against the real 1.71.0 toolchain.
- `cargo +nightly miri test --lib -- test_as_ref test_as_mut test_into
  test_from`: 24/24 pass, both with and without
  `MIRIFLAGS="-Zmiri-strict-provenance"`. (The full unfiltered `--lib`
  Miri run has 2-3 unrelated, pre-existing, non-deterministic
  `assert_ulps_eq!` failures in `slerp` -- confirmed by re-running the
  identical command against the pre-guard commit, same failure class
  recorded in `docs/baseline.md` for `rotate_from_euler::test_y` -- not
  a regression, filtered out of this specific check for signal clarity.)
- `mod tuple_layout_guard_tests`'s 3 tests: pass under both plain
  `cargo test` and `cargo +nightly miri test` with strict provenance.
- Release-build disassembly (the authoritative check per the study's own
  brief -- a microbenchmark's noise floor would have been too coarse to
  trust either way): a probe crate depending on `cgmath-next` via `path`,
  compiled `--release` with `opt-level = 3`, disassembled with `otool
  -tV` (aarch64). **First attempt failed this check**: the guard's
  offset computation constant-folded to literal values as hoped, but
  comparing two `[usize; 4]` arrays with `==` left a real runtime
  load+compare+branch in the compiled output for `Vector4<f32>::as_ref`
  -- a genuine, if small, non-zero cost, not a rounding error. Rewritten
  as a chain of individually-compared scalar offsets (`a == b && c == d
  && ...` instead of `[a,c] == [b,d]`); re-disassembled, and this form
  **fully constant-folds away** -- `Vector4<f32>::as_ref`,
  `Vector4<f64>::as_mut`, and `Quaternion<f32>::as_ref` all compile to
  pure arithmetic with zero guard overhead in the release build (the
  `Quaternion` and first `Vector4` probes even end up byte-identical and
  get linker-merged into one symbol, which is itself confirmation the
  Quaternion guard folded away too -- differing guard overhead would have
  produced different machine code). The array-vs-chain distinction is
  recorded on `tuple_layout_matches!`'s doc comment so it isn't
  accidentally reintroduced.
- Public API diff: regenerated via the same `cargo +nightly rustdoc
  --all-features -- -Zunstable-options --output-format json` method as
  `docs/api-inventory.md`. **Still zero-line diff, 2867/2867 entries
  match** -- the guard is entirely internal to function bodies, no
  signature changed.
- All 6 pairwise feature combinations (`docs/compatibility.md`) re-run:
  still 0 failures.
- All 5 reverse-dependency fixtures re-verified against the guard-updated
  source, not just a sample: `arcball` (`cargo check`, clean),
  `crevice` (13/13), `truck-base` (55/55, after re-applying the same
  documented 4-line qualification fix -- unrelated to this change, see
  `compat/fixtures/reverse-deps/RESULTS.md`), `vector-traits` (13/13),
  `three-d` (`cargo check`, clean). Plus the `dual-dep` differential
  suite against real `cgmath` 0.18.0 (9/9, bit-identical numeric output).
  All still pass unmodified.

### Remaining assumptions and limits

- **This is a tripwire, not a soundness proof.** Tuple layout is still
  officially unspecified by the Rust reference; the guard changes the
  failure mode of a hypothetical future divergence, it does not make the
  transmute itself language-guaranteed.
- **Scope is exactly UNSAFE-002.** This guard does nothing for
  UNSAFE-001, UNSAFE-003, or UNSAFE-004 -- those are unaffected, tracked
  separately below.
- **The guard itself adds a small amount of new `unsafe`** (`addr_of!` on
  `MaybeUninit` storage, in `tuple_layout_matches!` and
  `quaternion_tuple_layout_matches`). This pattern is sound by
  construction -- `addr_of!` never reads the pointee or creates a
  reference, confirmed clean under Miri with strict provenance -- and is
  a well-established technique (it's essentially what `offset_of!` and
  crates like `memoffset` do internally). Trading a small amount of
  narrowly-scoped, mechanically-verified unsafe for eliminating silent-UB
  risk in a much larger, unverifiable unsafe surface is the actual trade
  being made here, and is recorded as such rather than presented as "zero
  unsafe added."
- **Every call is checked, not just the first.** There's no
  once-per-process caching; each call recomputes and re-verifies. This
  was a deliberate choice matching the study's brief (a release-effective
  check, not a debug-only one) and is what made the constant-folding
  question meaningful to ask in the first place -- confirmed to be free
  when it folds, and correctly loud (not silently skipped) on platforms
  where it somehow doesn't.

**Status:** upgraded from "flagged, unverified" to **guarded and
audited** -- the silent-UB failure mode is now a verified, zero-cost (in
the matching case, confirmed via disassembly) runtime tripwire instead of
an unverifiable assumption. The underlying language-level fact that plain
tuple layout is unspecified is unchanged and cannot be fully closed
without removing the reference-returning conversions (a public API
change, still out of scope). Independently corroborated as a known,
upstream-unresolved issue (`rustgd/cgmath#538`), not something this fork
introduced or overlooked.

---

### UNSAFE-003: `det_sub_proc` (resolved -- was `det_sub_proc_unsafe`)

**A one-time feasibility study, explicitly scoped**, same category as the
UNSAFE-002 layout guard: replace `get_unchecked` with plain bounds-checked
indexing in this one function only, measure the actual release-build cost
via disassembly (not a hand-wave), and adopt only if the cost is zero or
negligible. **Answer: zero cost, confirmed by byte-identical disassembly,
not estimated.** `det_sub_proc_unsafe` is renamed `det_sub_proc`, is no
longer `unsafe fn`, and is removed from this audit's unsafe inventory --
this category no longer exists in the crate.

**File/Function (before):** `src/matrix.rs` (`unsafe fn
det_sub_proc_unsafe`), called from `Matrix4::determinant` (unconditional)
and `Matrix4::invert`'s `#[cfg(feature = "simd")]` branch (dead code, see
UNSAFE-004).

**What changed:** every `*s.get_unchecked(N)` became `s[N]` (mechanical,
`sed`-equivalent substitution, verified by diffing the transformed source
against the original with only that substitution applied); the function's
`unsafe fn` and every caller's wrapping `unsafe { }` block were removed,
since indexing is the only operation the function ever performed --
nothing else in its body was unsafe.

**Why zero cost, not just "likely":** every call site passes a literal
constant for `x`/`y`/`z` (`1,2,3` / `0,3,2` / `0,1,3` / `0,2,1`), so LLVM
can prove every derived index (`x`, `4+x`, `8+x`, `12+x`, etc.) is in
`0..16` at compile time and elide the bounds check entirely -- the
original audit predicted this ("almost certainly optimized away") but had
not measured it. This session measured it: a probe crate (path-dependency
on this crate, same technique as the UNSAFE-002 disassembly check) calling
the public `Matrix4::<f64>::determinant()`/`invert()` entry points was
compiled `--release` and disassembled with `otool -tV` (aarch64) both
before and after the substitution. **The two disassemblies are
byte-for-byte identical** (`diff` reports zero differences in the
instruction stream, the only line that differs is an unrelated absolute
file-path string embedded by the linker) -- not "similar," not "same
instruction count," literally the same machine code. This is a stronger
result than the UNSAFE-002 guard's (which added new, always-executed
comparisons that had to be shown to fold away): here there was nothing to
fold away because the safe and unsafe forms already lower to identical
codegen once bounds are provably static.

**Verification performed:**
- `cargo test --all-features`: all determinant/invert tests pass
  (`matrix2/3/4::test_determinant`, `matrix2/3/4::test_invert`,
  `test_invert_basis2/3`, transform's `test_invert`), same results as
  before the change.
- `cargo +nightly miri test --test matrix -- determinant invert`: 6/6
  pass, no UB reported.
- Release-build disassembly diff: zero instruction-level difference (see
  above).
- Full `cargo test --all-features`: 320/320 pass (306 pre-existing + 14
  new UNSAFE-001 tests from this session), 0 regressions.
- `cargo clippy --lib --tests --all-features`: no new warnings introduced
  by this change (the only warning touching this file, `unexpected cfg
  condition value: simd`, is the pre-existing UNSAFE-004 dead-code
  finding, unrelated).

**Caveat still true:** `invert`'s `#[cfg(feature = "simd")]` branch (now
calling the safe `det_sub_proc`) remains dead code in any standard build,
same as before -- this change doesn't make it reachable, just removes one
more `unsafe` from what would run if it ever were (see UNSAFE-004 below).

**Tests:** `tests/matrix.rs` `test_determinant`, `test_invert` (pre-existing
upstream tests, unmodified).

**Status:** resolved. No longer part of the unsafe inventory -- the crate
now has 3 remaining unsafe pattern groups (UNSAFE-001, UNSAFE-002,
UNSAFE-004), not 4.

---

### UNSAFE-004: `mem::uninitialized` + external `simd` crate load/store (resolved -- deleted)

**A one-time disposition decision, explicitly scoped:** don't force a Miri
test onto genuinely unreachable code (there is nothing to exercise it
with), and don't keep treating "audited" as equivalent to "acceptable to
leave in place" for a pattern built on `mem::uninitialized`, a
deprecated-since-1.39 API that is unsound for most types the moment it's
called, independent of whether the specific instantiation here happened to
be defensible. Since this code was private, unreachable from any declared
Cargo feature (`simd` was never a resolvable feature -- no `[features]
simd = [...]` entry ever existed), and would have required hand-editing
`Cargo.toml` and inventing a feature from scratch to ever compile, deletion
was the first candidate per that policy, and it's what happened.

**Deleted this session:** `src/quaternion_simd.rs` (157 lines) and
`src/vector_simd.rs` (417 lines) in full -- these were the only files
containing the `mem::uninitialized`-based SIMD load/store pattern. Also
removed as a direct, required consequence (not scope creep: leaving them
would either dangle or immediately trip `unused_macro_rules`/reference a
deleted file):
- `src/lib.rs`: `extern crate simd;` and the `mod quaternion_simd;`/
  `mod vector_simd;` declarations.
- `src/macros.rs`: the `impl_operator_simd!` macro, which had no callers
  left anywhere in the crate once the two files above were gone.
- `tests/vector4f32.rs`: a `#[cfg(feature = "simd")]` test block exercising
  `Vector4::sqrt_element_wide`/`recip_element_wide`/`rsqrt_element_wide`,
  methods that existed only in the now-deleted `vector_simd.rs` and would
  no longer resolve.

**Deliberately left alone (still dead, but out of this item's scope --
not `unsafe`, not this pattern, and touching them means auditing unrelated
macro infrastructure used far beyond this feature):**
- `src/lib.rs:53`, `#![cfg_attr(feature = "simd", feature(specialization))]`
- `src/macros.rs`'s `default_fn!` macro's `#[cfg(feature = "simd")]` branch
  (its `#[cfg(not(feature = "simd"))]` branch is the one actually used, in
  29 call sites across `vector.rs`/`quaternion.rs`/`macros.rs` -- unrelated
  to `mem::uninitialized`, contains no `unsafe`).
- `src/matrix.rs`'s two `#[cfg(feature = "simd")]` dead branches (an
  alternate `Matrix4::invert` using the now-safe `det_sub_proc`, and an
  alternate `Mul<Vector4<S>> for Matrix4<S>`) -- neither contains
  `unsafe`, both already correctly documented as dead code before this
  session, unaffected by this deletion.
- `Cargo.toml`'s commented-out `#simd = { version = "0.2", optional = true }`
  line and its "disabled indefinitely" comment -- this is upstream's own
  historical note (see `docs/provenance.md`), left as-is rather than
  scrubbed, same treatment given to other inherited-but-inert content.

**Verification performed:**
- `cargo build --all-features` / `cargo test --all-features`: 320/320
  pass, 0 regressions (same count as before this deletion -- nothing in
  the deleted files was ever compiled or counted in any real build).
- `cargo clippy --lib --tests --all-features`: no new warnings; in
  particular, no `unused_macro_rules` warning, confirming
  `impl_operator_simd!` had no remaining callers before it was removed
  (if it had, clippy/rustc would have flagged an unresolved macro
  reference instead) and no unused-macro warning was introduced.
- `cargo +nightly miri test --test soundness` (36/36) and
  `cargo +nightly miri test --test matrix` (86/87, the 1 failure is the
  pre-documented `rotate_from_euler` float non-determinism, unrelated):
  unaffected, same results as before this deletion.
- All 6 pairwise feature combinations plus the plain `--no-default-features`
  build: 0 failures across all 7 configurations.
- **Public API diff:** regenerated the rustdoc JSON and re-diffed the
  `cgmath.*`-namespace path list against the pristine 0.18.0 baseline --
  **zero diff, before and after this deletion.** This confirms what the
  reachability analysis already implied: none of the deleted `pub fn`s
  (`sqrt_element_wide` etc.) were ever part of the compiled public API
  surface under any real feature combination, so their removal changes
  nothing observable from outside the crate.

**Status:** resolved. Deleted, not just left flagged -- the crate no
longer contains a `mem::uninitialized` call anywhere
(`grep -rn "mem::uninitialized" src/` returns nothing). Remaining
`#[cfg(feature = "simd")]` dead branches elsewhere in the crate (listed
above) contain no `unsafe` and are out of this audit's scope.

---

## `#![deny(unsafe_op_in_unsafe_fn)]`

Not added to the crate root yet. **Update:** since UNSAFE-003's
`det_sub_proc_unsafe` -- the only remaining `unsafe fn` in the crate --
was resolved into a safe `fn` this session, there are now **zero
`unsafe fn` in the crate** (confirmed: `grep -rn "unsafe fn" src/`
returns nothing). `#![deny(unsafe_op_in_unsafe_fn)]` only changes
behavior inside `unsafe fn` bodies (it requires an explicit inner
`unsafe {}` around unsafe operations rather than inheriting the
function's own unsafe scope); with none left, adding the lint would now
be a zero-source-change, purely preventive addition rather than
requiring the edition bump or call-site rewiring described above. Not
added this session (out of scope for the UNSAFE-001/003 work actually
requested), but flagged as newly low-cost, worth doing opportunistically
in a future pass rather than deferred to an edition bump.

## SAFETY comments

None of the four groups above have inline `// SAFETY:` comments in the
source yet (upstream 0.18.0 never had them either, aside from the
now-deleted sarcastic one in `Array::swap_elements`). AGENTS.md requires
specific, non-generic `SAFETY:` comments on every *remaining* unsafe block.
This session prioritized removing the swap-family unsafe over annotating
what's left; adding `SAFETY:` comments to UNSAFE-001 through UNSAFE-004
(content drawn directly from the "Safety invariant" fields above) is
listed as next recommended work.
