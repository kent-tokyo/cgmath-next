# Compatibility notes

## Feature inventory (AGENTS.md §7.3)

The published 0.18.0 manifest (`Cargo.toml.orig`, imported verbatim as this
repo's `Cargo.toml`) declares only two features explicitly:

```toml
[features]
unstable = []
swizzle = []
```

`serde`, `mint`, and `rand` are **not** declared in `[features]`. They exist
as feature names only because each is an `optional = true` dependency of the
same name, and Cargo auto-generates an implicit feature
(`"<name>" = ["dep:<name>"]`) for every optional dependency that isn't
explicitly folded into another feature. Confirmed two ways:

* crates.io's own version API for 0.18.0 reports
  `"features":{"swizzle":[],"unstable":[]}` — mint/rand/serde absent.
* `cargo metadata --no-deps` on the imported source resolves the full
  feature set as `mint`, `rand`, `serde`, `swizzle`, `unstable`, where
  `mint`/`rand`/`serde` each map to `["dep:<name>"]` (Cargo-synthesized) and
  `swizzle`/`unstable` map to `[]` (author-written).

This distinction matters for AGENTS.md §7.3's instruction not to add a
feature "just because it exists on master": all five names above already
exist in 0.18.0's resolved feature set today, so all five are in scope
as-is. No feature is being added.

`unstable` has no `#[cfg(feature = "unstable")]` gate anywhere in `src/`
(confirmed by grep) — same on 0.18.0 as it is upstream. Its only observable
effect in 0.18.0 is enabling a `#[cfg(feature = "simd")]` block, but `simd`
itself is not a real, resolvable feature (there is no `simd` dependency or
`[features] simd = […]` entry — the `simd` optional dependency is commented
out in `Cargo.toml.orig`: `#simd = { version = "0.2", optional = true }`).
This produces the `unexpected cfg condition value: simd` clippy warning seen
in the baseline (`docs/baseline.md`). Per §12, this is upstream's existing
behavior — `unstable`'s name is kept as-is, dead gate included, because
removing it would be a functional/API change beyond this phase's scope.

| Feature | Declared how | Gates | Extra dependency |
|---|---|---|---|
| `swizzle` | explicit, `[features]` | swizzle methods/macros | none |
| `unstable` | explicit, `[features]` | nothing reachable (dead `cfg(feature="simd")` only) | none |
| `serde` | implicit (optional dep) | `Serialize`/`Deserialize` impls | `serde` |
| `mint` | implicit (optional dep) | `mint` conversions | `mint` |
| `rand` | implicit (optional dep) | `Distribution` impls | `rand` |

## `[lib] name` resolution (AGENTS.md §4)

§4 explicitly forbids guessing this setting and requires empirical fixtures
against five conditions. All five were built under `compat/fixtures/` and
run against the renamed package (`name = "cgmath-next"` in `[package]`,
`name = "cgmath"` kept in `[lib]`, unchanged from upstream's own
`Cargo.toml.orig`).

**Finding: keep `[lib] name = "cgmath"` exactly as upstream already declares
it. Do not remove it, do not change it.** This is the only setting that
satisfies all five conditions — verified empirically both ways (fixtures
passing with it kept, and a negative-control run proving failure without
it), not assumed.

### How the two settings differ, mechanically

* `[lib] name` controls the compiled crate's own name — this is what
  `cargo test`'s in-crate integration tests (`tests/*.rs`) and doctests link
  against when the crate tests itself. It has nothing to do with how
  *external* consumers reference the dependency.
* An external consumer's import path is controlled by their own Cargo.toml
  dependency table key (`cgmath = { package = "cgmath-next", ... }` lets
  them write `use cgmath::...`; a plain `cgmath-next = "0.18.1"` entry lets
  them write... see fixture 2 below, which turned up a result worth noting).

### Fixture matrix and results

| # | Fixture (`compat/fixtures/...`) | Setup | `[lib] name = "cgmath"` (kept) | `[lib] name` omitted (negative control, not committed) |
|---|---|---|---|---|
| 1 | `rename-dep` | `cgmath = { package = "cgmath-next", path = ".." } ` + `use cgmath::{Matrix4, Quaternion, Vector3};` | **pass** — `cargo test` ok | not tested (external-rename path is unaffected by `[lib] name`) |
| 2 | `plain-dep` | `cgmath-next = { path = ".." }` (no rename) + `use cgmath::Vector3;` | **pass** — `cargo test` ok, surprising but correct: `[lib] name` governs the identifier regardless of the dependency key | would need `use cgmath_next::Vector3;` instead — not the upstream-compatible path |
| 3 | `dual-dep` | real `cgmath = "0.18.0"` **and** `cgmath_next = { package = "cgmath-next", path = ".." }` in one manifest | **pass** — both resolve, no symbol collision, values agree | not tested |
| 4 | `workspace-rename` | fixture 1's setup, but `member/` is a workspace member under a `[workspace]` root | **pass** — identical behavior in vs. out of a workspace | not tested |
| 5 | root crate's own `cargo test --doc` (22 tests) and `cargo test --test matrix` (87 tests), unmodified `use cgmath::...` source | in-crate self-test | **pass**, all 22 doctests + all 87 matrix tests, zero source changes | **fails**: all 22 doctests fail with `error[E0433]: cannot find module or crate 'cgmath' in the crate root` / `error[E0432]: unresolved import 'cgmath'`, because the crate compiles itself under the name `cgmath_next` (package name with `-`→`_`) with no explicit override |

Fixture 5's negative control (run in an isolated `/tmp` copy, not part of
this repository) is the direct confirmation of the "doctest trap": deleting
the `[lib] name = "cgmath"` line and letting it default from the package
name breaks every doctest and in-crate integration test that still says
`use cgmath::...` — which is all of them, since Phase 1 forbids editing
upstream test/doctest source. Keeping the line upstream already wrote is
what makes "upstream tests pass unmodified" (§8 completion condition)
possible at all.

### Condition-by-condition (§4)

1. **`cgmath`という依存名でimportできる** — yes, fixture 1.
2. **`cgmath-next`単独でも通常利用できる** — yes, fixture 2; and notably the
   import path is still `cgmath::...` even without an explicit rename, since
   that's what `[lib] name` fixes it to.
3. **旧`cgmath`と新crateをrenameして同時に依存できる** — yes, fixture 3.
4. **doctestでも同じ挙動になる** — yes, fixture 5 (root crate's own doctests).
5. **workspace内外で挙動が変わらない** — yes, fixture 4 vs. fixture 1.

## Feature matrix (AGENTS.md §12)

Each declared/implicit feature tested individually with
`cargo test --no-default-features --features <name>`, plus a no-features
baseline and the existing `cargo test --all-features` from `docs/baseline.md`.

| Row | Command | Result |
|---|---|---|
| no features | `cargo test --no-default-features` | pass, 300 tests, 0 failed |
| `serde` alone | `cargo test --no-default-features --features serde` | pass, 301 tests, 0 failed (+1: a serde-gated doctest) |
| `mint` alone | `cargo test --no-default-features --features mint` | pass, 300 tests, 0 failed |
| `rand` alone | `cargo test --no-default-features --features rand` | pass, 300 tests, 0 failed |
| `swizzle` alone | `cargo test --no-default-features --features swizzle` | pass, 302 tests, 0 failed (+2: `tests/swizzle.rs` unlocked) |
| `unstable` alone | `cargo test --no-default-features --features unstable` | pass, 300 tests, 0 failed (confirms `unstable` gates no reachable code either way, see the feature inventory above) |
| all features | `cargo test --all-features` | pass, 303 tests, 0 failed (300 base + 1 serde doctest + 2 swizzle tests, additive; `docs/baseline.md`'s "281" is the same command run in Phase 1, before `tests/soundness/`'s 22 tests existed — 281 + 22 = 303, consistent, not a discrepancy) |

The base count is 300 (256 upstream + 22 soundness + 22 doctest); two rows
add to that where a feature unlocks extra gated tests/doctests, per above.
Raw logs backing these counts: `/tmp/feature-matrix/*.log` (not committed,
machine-local scratch output — reproduce with the commands in the table).
No feature combination fails, and none was skipped.

### Pairwise feature combinations

All C(4,2) = 6 pairs among `serde`/`mint`/`rand`/`swizzle` (`unstable`
excluded — confirmed above to gate no reachable code, so pairing it with
anything is equivalent to the single-feature row), run as
`cargo test --no-default-features --features <a>,<b>`:

| Pair | Result |
|---|---|
| `serde,mint` | pass, 301 tests, 0 failed |
| `serde,rand` | pass, 301 tests, 0 failed |
| `serde,swizzle` | pass, 303 tests, 0 failed |
| `mint,rand` | pass, 300 tests, 0 failed |
| `mint,swizzle` | pass, 302 tests, 0 failed |
| `rand,swizzle` | pass, 302 tests, 0 failed |

No `cfg` logic in `src/lib.rs` makes any feature reference or gate on
another (confirmed by reading the file, same check as the feature
inventory section above) — that's the actual reason no pairwise-specific
interaction is possible here, not the test run itself. The run's additive
counts (every pair's total is exactly the sum of that pair's individual-row
bonuses: serde +1 doctest, swizzle +2 tests, mint/rand +0) are consistent
with that and rule out a pair silently *skipping* a test some other row
runs, but a passing count can't prove the *absence* of a behavioral
interaction on its own — only that nothing gated in or out. §12's
"individual and combination" requirement is now fully met at the level
this test suite can check, not just individual + all-features.

## Differential testing vs. upstream (AGENTS.md §11.2)

`compat/fixtures/dual-dep/` depends on real `cgmath = "0.18.0"` from
crates.io and `cgmath-next` (via `path`) simultaneously, under distinct
local names, and runs the same inputs through both. 9 numeric-differential
tests, all exact `==` comparisons (not approx): vector add/sub/mul/div,
dot/cross/magnitude/normalize, matrix add/sub/mul/transpose/determinant/
invert, quaternion multiplication and vector rotation, Euler-to-matrix/
quaternion conversion, Deg/Rad conversion, nlerp/slerp interpolation,
point midpoint, `look_at_rh`, perspective/ortho projection, and
`Decomposed` transform composition. **9/9 pass, bit-identical output.**

Plus 6 serde wire-format differential tests (both crates built with
`features = ["serde"]`): `Vector1..4`, `Point1..3`, `Matrix2..4`,
`Quaternion`, `Euler<Rad<S>>`/`Euler<Deg<S>>`, and `Decomposed`, each
across `f32` and `f64` where applicable. Each test asserts, not just
"parses OK" but actual equality: (1) `serde_json::to_string` output is
byte-for-byte identical between the two crates, (2) each crate's own
output round-trips back to an equal value, and (3) each crate's JSON
cross-deserializes correctly into the *other* crate's type. **All pass,
byte-identical.** This is expected, not incidental: every serde-derived
type in both crates uses plain public named fields with no `#[serde(...)]`
attribute overrides, so field name/order/type is what determines the wire
format, and the zero public-API-diff result (`docs/api-inventory.md`)
already establishes those match.

Plus 5 mint conversion-inventory tests (both crates built with
`features = ["mint"]`), covering every mint impl in the crate --
`Vector2..4`, `Point2..3`, `Matrix2..4`, `Quaternion`, and `Euler` --
each bidirectional (`Into`/`From`). Every test uses pairwise-distinct
component values specifically so that a component swap, a matrix
transpose, or a quaternion scalar/vector mixup would fail the assertion
rather than pass by coincidence (symmetric test data can't distinguish
"correct" from "silently reordered"). The matrix test additionally
cross-checks against `Matrix::row()` on a deliberately asymmetric
matrix and asserts the mint column does *not* equal the row, positively
ruling out an accidental transpose rather than just checking the column
happens to be right. **All 5 pass**, both against real `cgmath` 0.18.0
and `cgmath-next`, confirming `mint::ColumnMatrix{2,3,4}` really is
column-major relative to cgmath's own layout, `mint::Quaternion.s`/`.v`
really are the scalar/vector parts (not swapped), and
`mint::EulerAngles.a/b/c` really map to `x/y/z` in that order.

`tests/soundness/`
already independently covers `swap_columns`/`swap_elements`/`swap_rows`
behavior (not compared against upstream here, since upstream's version of
those functions is the known-UB one -- see `docs/unsafe-audit.md` and the
fix commit for why comparing against upstream's behavior for those
specific functions isn't the right test).

## `swizzle` feature: API diff ON vs. OFF, vs. upstream

Swizzle methods (`.xy()`, `.xxyz()`, etc.) are entirely generated by
`build.rs` at compile time (`gen_swizzle_functions`, unmodified this
session) -- gated behind `#[cfg(feature = "swizzle")]` in `build.rs`
itself, not just at the call site, so when the feature is off the
generator function returns an empty string and zero swizzle methods are
emitted into the macro `build.rs` writes to `OUT_DIR`.

**Machine-verified inventory, not a spot check.** Generated rustdoc JSON
(`cargo +nightly rustdoc --features swizzle -- ...`) for both
`cgmath-next` and pristine `cgmath` 0.18.0 (`_extract/cgmath-0.18.0/`),
extracted every item whose doc comment starts with `"Swizzle operator"`
(the exact string `build.rs` generates), keyed by
`(source file, method name, return type)`:

- **ON: 550 swizzle methods in both crates.** Diffed as sets --
  **zero methods only in `cgmath-next`, zero only in 0.18.0.** Every
  name, return type, and source location matches exactly.
- **OFF (`--no-default-features` for `cgmath-next`): 0 swizzle methods.**
  Confirms the feature gate actually removes the methods from the
  compiled API, not just from documentation.
- Breakdown by return type confirms every dimension and every
  duplicate-component case is covered: `Point1<S>`: 6, `Point2<S>`: 14,
  `Point3<S>`: 36 (Points, 1-3 dimensional); `Vector1<S>`: 10,
  `Vector2<S>`: 30, `Vector3<S>`: 100, `Vector4<S>`: 354 (Vectors, 1-4
  dimensional) -- 550 total. Sample names include duplicate-component
  swizzles like `ww`, `www`, `wwww`, confirming those aren't excluded.

**Compile-fail fixture:** `compat/fixtures/swizzle-off/` depends on
`cgmath-next` with the `swizzle` feature deliberately *not* enabled and
calls `.xy()` on a `Vector2`. `cargo build` in that directory is expected
to fail -- and does, with `error[E0599]: no method named `xy` found for
struct `Vector2<S>` in the current scope`. Positive control: the same
fixture builds cleanly with `cargo build --features cgmath-next/swizzle`,
confirming the failure is specifically about the feature being off, not
some unrelated typo or path issue. This directly proves (not just infers
from the rustdoc-JSON absence above) that the swizzle API doesn't leak
into a build that never requested it.

## `rand` feature verification

Unlike `serde`/`mint` above, `rand`'s `Distribution` impls aren't
byte-for-byte comparable at runtime -- a random sample is, by design, not
a deterministic function of its inputs, so an exact-value differential
test against upstream would either be meaningless (different RNG state)
or would end up asserting an exact output sequence upstream never
documented or guaranteed for a given seed. Verified instead:

- **Every `Distribution` impl is source-identical to pristine 0.18.0.**
  All 7 impl sites (`Vector1..4` via `impl_vector!`, `Matrix2/3/4`,
  `Quaternion`, `Rad`/`Deg` via `impl_angle!`, `Euler`) diffed
  byte-for-byte against `_extract/cgmath-0.18.0/` -- zero differences in
  any impl body, not just "the same types implement the trait". This
  crate has never touched rand-related code, so this is expected, but it
  was verified rather than assumed.
- **`tests/rand_distribution.rs`** (new, gated `#![cfg(feature = "rand")]`,
  6 tests): confirms the existing contract these impls have always had --
  every generated component is finite, and within its documented or
  source-derivable range: `[0, 1)` for `Vector`/`Matrix`/`Quaternion`
  components -- confirmed against rand 0.8.7's own source, not just
  inferred: `distributions::Standard`'s docs state "samples from `[0, 1)`"
  and its `Distribution<f32>`/`Distribution<f64>` impl macro carries the
  comment `// Multiply-based method; 24/53 random bits; [0, 1) interval.`
  (`rand-0.8.7/src/distributions/float.rs`), which these impls compose via
  `rng.gen()` -- `[-π, π)` for `Rad` and
  `[-180, 180)` for `Deg`/`Euler` (the literal bounds `impl_angle!` passes
  to `rng.gen_range`). Plus a non-degeneracy sanity check (not a
  statistical RNG-quality test, which would be flaky by nature -- just
  confirming samples aren't all identical or all zero, which would
  indicate broken RNG wiring). **6/6 pass.**
- **Feature isolation**: `cargo tree` (no flags, and
  `--no-default-features`) shows `rand` entirely absent from the
  dependency graph, same check as `serde`/`mint`.

## Reverse-dependency fixtures (AGENTS.md §14)

See `compat/fixtures/reverse-deps/RESULTS.md` for the full record. 5 real
crates.io reverse-dependencies verified against `cgmath-next` via the
dependency-rename mechanism: `arcball` (camera control, pass, 0 changes),
`crevice` (GPU/GLSL layout + mint interop, pass, 0 changes), `truck-base`
(CAD kernel base, pass after 4 one-line changes -- a genuinely interesting
multi-crate rename propagation case involving a third-party `cgmath`
extension crate, not a `cgmath-next` compatibility gap; see the linked doc
for the full diagnosis), `vector-traits` (multi-backend vector-math trait
abstraction, pass, 0 changes -- plus a compile-time type-identity check,
not just a compile-success check, confirming values constructed via a
direct `cgmath-next` dependency are the *identical* nominal type as what
the extension crate's trait impls expect once it migrates too, and
correctly fail with a plain `E0277` trait-bound error, not a silent type
split, when it hasn't), and `three-d` (2D/3D rendering engine, pass, 0
changes -- a heavier, non-optional, feature-ungated `cgmath` consumer).

This meets both the §20 alpha gate (3+) and the §20 stable gate (5+).
