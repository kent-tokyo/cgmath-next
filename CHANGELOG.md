# Change Log

All notable changes to this project will be documented in this file, following
the format defined at [keepachangelog.com](http://keepachangelog.com/).
This project adheres to [Semantic Versioning](http://semver.org/).

Entries below `[v0.18.0]` are `cgmath`'s own changelog, imported verbatim
as part of the faithful 0.18.0 import (see `docs/provenance.md`).
`cgmath-next`-specific entries start at the top.

## [Unreleased] (cgmath-next)

## [0.18.1] (cgmath-next) - 2026-08-17

### Fixed

 - **Soundness**: `Matrix{2,3,4}::swap_columns` and `Matrix{2,3,4}::swap_elements`
   could reach undefined behavior from 100% safe Rust
   ([RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html),
   [rustgd/cgmath#565](https://github.com/rustgd/cgmath/issues/565)). Fixed
   by replacing the `unsafe { ptr::swap(...) }` pattern with a safe
   read-into-temporary-then-write sequence.
   **This fix's scope is broader than the advisory's literal wording**: the
   advisory names only same-index `swap_columns` calls; this project found
   and fixed the same bug (a) for `swap_columns` with *any* two indices,
   not just equal ones, since two sequential `IndexMut` reborrows of the
   same matrix are unsound regardless, and (b) in two more call sites the
   advisory doesn't mention, `Array::swap_elements` (shared by
   `Vector2/3/4`, `Point2/3/4`, and `Matrix::swap_rows`) and `Matrix`'s
   own `(col, row)` `swap_elements`. See `docs/unsafe-audit.md` and the
   commit history for detail. `rustgd/cgmath#565` is still open upstream
   with `patched = []` in the advisory as of this writing.

### Changed

 - **UNSAFE-002 (tuple-layout transmute) is now guarded.** `AsRef`/
   `AsMut`/`From<&Tuple>`/`From<&mut Tuple>` conversions between
   `Vector1..4`/`Point1..3`/`Quaternion` and their homogeneous-tuple forms
   now verify size, alignment, and per-field byte offsets at runtime
   before transmuting, and panic instead of transmuting if they don't
   match. Confirmed via release-build disassembly to compile away to zero
   overhead when layout matches (as it does on every platform tested).
   This is a runtime tripwire, not a language-level soundness proof —
   plain tuple layout remains unspecified by the Rust reference; see
   `docs/unsafe-audit.md`'s feasibility-study section for exactly what is
   and isn't established, and for the MSRV-compatibility analysis behind
   this specific implementation choice.
 - **UNSAFE-003 (`det_sub_proc_unsafe` unchecked indexing) is resolved.**
   Replaced with plain bounds-checked indexing (`s[i]` instead of
   `s.get_unchecked(i)`); confirmed via release-build disassembly to
   compile to byte-identical machine code to the unchecked version, i.e.
   zero cost, not an estimate. Renamed to `det_sub_proc` and no longer an
   `unsafe fn` — the crate now has zero remaining `unsafe fn`.
 - **UNSAFE-004 (`mem::uninitialized` + `simd` crate load/store) is
   resolved by deletion.** `src/quaternion_simd.rs` and
   `src/vector_simd.rs` — private, permanently unreachable from any
   declared Cargo feature — are removed, along with their `mod`
   declarations, the now-unused `impl_operator_simd!` macro, and a test
   block referencing their methods. Zero public API diff confirmed
   before/after. `docs/unsafe-audit.md` now tracks 2 remaining unsafe
   pattern groups (`UNSAFE-001`, `UNSAFE-002`) instead of 4.
 - Added a dedicated Miri regression suite for UNSAFE-001 (fixed-size-array
   reference conversions), `tests/soundness/array_conversions.rs`: covers
   `AsRef`/`AsMut`/`From<&[..]>`/`From<&mut [..]>` for `Vector1..4`,
   `Point1..3`, both `Matrix2..4` array shapes, and `Quaternion`, including
   write-back-through-the-view checks and `-Zmiri-strict-provenance`.
 - Extended `compat/fixtures/dual-dep/` with 6 `serde` wire-format
   differential tests against real `cgmath` 0.18.0: byte-for-byte JSON
   equality, same-crate round-trip, and cross-crate deserialization for
   `Vector1..4`, `Point1..3`, `Matrix2..4`, `Quaternion`,
   `Euler<Rad/Deg<S>>`, and `Decomposed`. All pass, byte-identical.
 - Extended `compat/fixtures/dual-dep/` with 5 `mint` conversion-inventory
   tests covering every mint impl (`Vector2..4`, `Point2..3`,
   `Matrix2..4`, `Quaternion`, `Euler`), using pairwise-distinct
   component values to positively catch a swap/transpose/scalar-vector
   mixup rather than pass on symmetric test data. Confirms
   `mint::ColumnMatrix{2,3,4}` is genuinely column-major (checked against
   `Matrix::row()` on an asymmetric matrix, not just the column), and
   `mint::Quaternion.s`/`.v` and `mint::EulerAngles.a/b/c` map correctly.
   All pass.
 - Added `tests/rand_distribution.rs` (6 tests): confirms every
   `Distribution` impl's generated values are finite and within their
   documented/derivable range. Every `Distribution` impl was also
   confirmed byte-for-byte identical to pristine `cgmath` 0.18.0's
   source. Deliberately doesn't assert an exact RNG output sequence,
   since upstream never guaranteed one for a given seed.
 - Verified the `swizzle` feature's public API via rustdoc-JSON inventory
   (550 methods with the feature on, 0 with it off, in both
   `cgmath-next` and pristine `cgmath` 0.18.0, zero difference either
   direction) and a new compile-fail fixture,
   `compat/fixtures/swizzle-off/`, which fails with `E0599` when the
   feature is off and builds cleanly when it's on.
 - Renamed package to `cgmath-next` (crates.io name only —
   `[lib] name = "cgmath"` is unchanged, so `use cgmath::...` still works).
   Version set to `0.18.1-alpha.1` to signal this is a patch series on top
   of the `cgmath` 0.18 API, not a new major version.
 - Pre-publish metadata fix: `homepage` pointed at the original
   `rustgd/cgmath` repository, which would have been misleading for this
   fork's own crates.io listing — now points at `cgmath-next`'s own repo.
   `authors` now credits both the original project and this fork.
 - `compat/fixtures/dual-dep` (serde/mint differential tests) and
   `compat/fixtures/swizzle-off` (feature-gate compile-fail fixture) now
   run as a blocking `compat` CI job on every push/PR, promoting that
   compatibility evidence from point-in-time verification to continuous
   regression protection.
 - Added a `serde`/`mint`/`rand` feature-leak check to the `compat` CI
   job, verified in both directions (absent with no features, present
   with `--all-features`) so the check can't pass vacuously.
 - CI hardening: `ci.yml` now declares explicit least-privilege
   `permissions: contents: read`, and every GitHub Action used in
   `ci.yml`/`publish.yml` is pinned to a full commit SHA rather than a
   mutable tag or branch (trailing comment names the original ref).
   `publish.yml` also gained a `permissions`/`concurrency` block and
   `Cargo.toml` gained `publish = ["crates-io"]`, restricting publish to
   the intended registry.
 - Published `0.18.1-alpha.1`, then `0.18.1` stable -- an alpha
   observation period surfaced no soundness or compatibility issues
   against it.

### Known compatibility gaps

 - See `docs/api-inventory.md` for the machine-verified public API diff
   (currently empty) and `docs/compatibility.md` for what has and hasn't
   been verified (feature matrix, differential tests, reverse-dependency
   fixtures).

### MSRV

 - See `docs/msrv.md`.

### Tested feature combinations

 - See `docs/compatibility.md`.

### Tested reverse-dependency fixtures

 - See `docs/compatibility.md` / `compat/fixtures/`.

## [v0.18.0] - 2021-01-03

### Changed

 - Refactored dependencies of experimental "specialization" feature into
   default_fn! macro to reduce code duplication and complexity. Currently
   only needed for non-functional SIMD feature.
 - Refactored SIMD code into separate source files. See README.md for details.
 - **Breaking**: Quaternion memory layout changed to `[x, y, z, w]`. The
   `From` and `Into` impls for `[S; 4]` and `(S, S, S, S)` have been changed
   accordingly.


### Added

 - Add `VectorN::zip` and `PointN::zip`
 
## [v0.17.0] - 2019-01-17

### Added

 - Add signed `Angle` normalization

### Changed

 - Move `lerp()` from `InnerSpace` to `VectorSpace`
 - `const` constructors

## [v0.16.1] - 2018-03-21

### Added

 - Implement `ElementWise` trait for point types
 - Add `map` function to points and vectors

### Changed

 - Remove `BaseNum` trait requirement for `PointN::new` functions

## [v0.16.0] - 2018-01-03

### Added

- Add `InnerSpace::project_on`
- Add `Array::len`
- Re-export `Bounded` and implement for vectors, points, and angles
- Add vector subtraction to `EuclideanSpace`
- Add swizzle functions behinde that `"swizzle"` feature
- Add `Matrix4::look_at_dir`

### Changed

- Return `Option` from cast functions

## [v0.15.0] - 2017-07-30

### Added

- Implement `mint` conversions behind a feature
- Add `Quaternion::cast`

### Changed

- Rename `use_simd` feature to `simd`
- Rename `eders` feature to `serde`

### Fixed

- Fix matrix inversions for small determinants

## [v0.14.1] - 2017-05-02

### Fixed

- Add a workaround for rust-lang/rust#41478, and in the process cleaned up
  some type projections for angles

## [v0.14.0] - 2017-04-26

## Changed

- Constrain `VectorSpace`, `Rotation`, and `Angle` by `iter::Sum`
- Constrain `SquareMatrix` by `iter::Product`

## [v0.13.1] - 2017-04-22

### Changed

- Update `serde` and `serde_derive` to version `1.0`.

## [v0.13.0] - 2017-04-14

### Added

- Add optional `use_simd` feature to improve the performance of `Vector4<f32>`,
  `Matrix4<f32>` and `Quaternion<f32>`. According to @DaseinPhaos in #394, under
  the given benchmark certain operations were able to become up to 60% faster.
- Add component wise casting for the matrix and point types

### Changed

- Update `serde` to version `0.9`, and use `serde_derive` instead of `serde_macros`.

## [v0.12.0] - 2016-09-14

### Changed

- Use [approx](https://github.com/brendanzab/approx/) for approximate equality
  comparisons
- Remove `#[repr(packed)]` from all structs where it was specified
- Update serde to 0.8

## [v0.11.0] - 2016-08-17

### Added

- `Quaternion::from_arc`

### Changed

- Change the angle types to be tuple structs
- Make from-angle constructors take generic `Into<Rad<S>>` values
- Fix `Decomposed::concat` implementation

## [v0.10.0] - 2016-05-11

### Added

- A `MetricSpace` trait for types that have a distance between elements.
- `EuclideanSpace::{midpoint, centroid}` functions with default
  implementations.
- `Vector1` and `Point1` structs.
- Serde support behind the `eders` feature flag.
- An `ApproxEq` implementation for `Decomposed`.

### Changed

- Depend on the `num-traits` crate rather than `num`, seeing as we only use the
  traits in `num`. `num_traits` has also been re-exported so that you can more
  easily use these in your project.
- Use an `Euler` type for euler angle conversions.
- Constrain `InnerSpace` by `MetricSpace`.
- Constrain `Rotation` by `One`
- Implement `Transform` and `Transform3` for `Matrix4`.
- Implement `Transform`, `Transform2`, and `Transform3` for `Matrix4`.
- Fix `Euler`-`Quaternion` and `Quaternion`-`Euler` conversions. The axes are
  now correct, and the angles are applied in _x_-_y_-_z_ order. The conversion now
  matches the conversion from axis angle.
- Fix `Euler`-`{Matrix3, Matrix4}` conversions.

## Removed

- `Rotation::transform_as_point`
- `AffineMatrix3`
- `Rotation::invert_self`
- `Matrix::invert_self`

## [v0.9.1] - 2016-04-20

### Changed

- Fix angle assignment operators so that they actually mutate `self`.

## [v0.9.0] - 2016-04-19

### Changed

- Assignment operators implementations have been stabilised, to coincide with
  their [stabilisation in Rust 1.8](http://blog.rust-lang.org/2016/04/14/Rust-1.8.html).
- Renames `Vector` trait to `VectorSpace`.
- Renames `EuclideanVector` to `InnerSpace`.
- Renames `Point` to `EuclideanSpace`, and `Point::Vector` to `EuclideanSpace::Diff`.
- `Quaternion`s now implement `VectorSpace` and `InnerSpace` for the functions
  they share.
- The `Matrix` trait is now constraint by `VectorSpace`, with `Matrix::Element`
  removed in favor of `VectorSpace::Scalar`.

## [v0.8.0] - 2016-04-06

### Added

- Implements `fmt::Debug` for `Basis2`, `Basis3`, and `AffineMatrix3`
- A `prelude` module for easy importing of common traits.
- Constrained conversion functions for assisting in situations where type
  inference is difficult.
- An `ElementWise` trait for non-mathematical element-wise operations.
- A default implementation for `EuclideanVector::angle`.

### Changed

- Improves the `fmt::Debug` impls for `Vector`, `Matrix`, `Point`, `Decomposed`,
  `Quaternion` and `Angle` to make them easier to derive, and have clearer
  formatting.
- Marks vectors, points, matrices, and angles as `#[repr(C, packed)]`.
- Renames the `Vector::{length, length2}` functions to `Vector::{magnitude, magnitude2}`.
- Move `Angle::new` to be directly implemented on the `Rad` and `Deg` types.
- Move `Vector::dot` to `EuclideanVector` trait.
- Move `Vector::from_value` to `Array` trait.

### Removed

- The non-mathematical operator trait implementations have been removed from
  the `Vector` trait, in favor of the `ElementWise` trait.
- `Angle::equiv`.
- Remove `neg_self` method on vectors and matrices.

## [v0.7.0] - 2015-12-23

### Added
- Add missing by-ref and by-val permutations of `Vector`, `Matrix`, `Point`,
  `Quaternion` and `Angle` operators.
- Ease lifetime constraints by removing `'static` from some scalar type
  parameters.
- Weaken type constraints on `perspective` function to take an `Into<Rad<S>>`.
- Add `Angle::new` for constructing angles from a unitless scalar.
- Implement assignment operators for nightly builds, enabled by the `"unstable"`
  feature.

### Changed
- `Vector`, `Matrix`, `Point`, and `Angle` are now constrained to require
  specific operators to be overloaded. This means that generic code can now use
  operators, instead of the operator methods.
- Take a `Rad` for `ProjectionFov::fovy`, rather than arbitrary `Angle`s. This
  simplifies the signature of `PerspectiveFov` from `PerspectiveFov<S, A>` to
  `PerspectiveFov<S>`.
- The following trait constraints were removed from `Angle`: `Debug`,
  `ScalarConv`, `Into<Rad<S>>`, `Into<Deg<S>>`.
- `Angle` no longer requires `One`, and the implementations have been removed
  from `Deg` and `Rad`. This is because angles do not close over multiplication,
  and therefore cannot have a multiplicative identity. If we were truly accurate,
  `Angle * Angle` would return an `Angle^2` (not supported by the current api).
- Make remainder operators on `Angle`s make sense from the perspective of
  dimensional analysis.
- Moved free trigonometric functions onto `Angle`.

### Removed
- Remove redundant `Point::{min, max}` methods - these are now covered by the
  `Array::{min, max}` methods that were introduced in 0.5.0.
- Removed `ToComponents`, `ToComponents2`, and `ToComponents3`. If you were
  relying on `ToComponents::decompose`, you can produce the same effect by
  accessing the fields on `Decomposed` directly. To create the scale vector,
  use: `Vector::from_value(transform.scale)`.
- Removed `CompositeTransform`, `CompositeTransform2`, and `CompositeTransform3`.
- Remove `Vector::one`. Vectors don't really have a multiplicative identity.
  If you really want a `one` vector, you can do something like:
  `Vector::from_value(1.0)`.
- Remove operator methods from `Vector`, `Matrix`, `Point`, and `Angle` traits
  in favor of operator overloading.
- Remove `*_self` methods from `Vector`, `Matrix`, `Point`, and `Angle`. The
  operator methods can be used via the unstable assignment operators.
- Remove `#[derive(Hash)]` from `Deg` and `Rad`. This could never really be used
  these types, because they expect to be given a `BaseFloat` under normal
  circumstances.

## [v0.6.0] - 2015-12-12

### Added
- This CHANGELOG for keeping track of notable changes.
- `Matrix4::{from_scale, from_nonuniform_scale}` for easily constructing
  homogeneous scale matrices.

### Changed
- Renamed `SquareMatrix::one` to `SquareMatrix::identity`. `identity` is easier
  to search for,
  and the more common name for the multiplicative identity for matrices.
- Matrix impls have now been constrained to `S: BaseFloat`.

## [v0.5.0] - 2015-11-20

### Changed
- Take many point and vector parameters by value.
- Take point and vector operator overloads by value.
- Divide `Matrix` trait into `Matrix` and `SquareMatrix`, opening the door for
  non-square matrices in the future.
- Make many trait type parameters associated types.
- Move element-wise methods from `Vector` and `Point` onto the `Array1` trait,
  and rename it to `Array`.
- Make pointer access methods on `Array` match the naming scheme of those in the
  standard library.

### Removed
- Removed collision types: `Ray`, `Plane`, `Frustum`, `Aabb2`, `Aabb3` `Obb2`,
  `Obb3` `Sphere`, `Cylinder`. These can now be found at
  [csherratt/collision-rs](https://github.com/csherratt/collision-rs).
- Remove `Array2` trait, moving methods onto the `Matrix` trait.

## [v0.4.0] - 2015-10-25

## [v0.3.1] - 2015-09-20

## [v0.3.0] - 2015-09-20

## [v0.2.0] - 2015-05-11

## [v0.1.6] - 2015-05-10

## [v0.1.5] - 2015-04-25

## [v0.1.4] - 2015-04-24

## [v0.1.3] - 2015-04-06

## [v0.1.2] - 2015-04-01

## [v0.1.1] - 2015-03-25

## [v0.1.0] - 2015-03-15

## [v0.0.8] - 2015-03-09

## [v0.0.7] - 2015-03-01

## [v0.0.6] - 2015-02-21

## [v0.0.5] - 2015-02-16

## [v0.0.4] - 2015-02-11

## [v0.0.3] - 2015-02-08

## v0.0.1 - 2014-06-24

[Unreleased]: https://github.com/brendanzab/cgmath/compare/v0.16.1...HEAD
[v0.16.1]: https://github.com/brendanzab/cgmath/compare/v0.16.0...v0.16.1
[v0.16.0]: https://github.com/brendanzab/cgmath/compare/v0.15.0...v0.16.0
[v0.15.0]: https://github.com/brendanzab/cgmath/compare/v0.14.1...v0.15.0
[v0.14.1]: https://github.com/brendanzab/cgmath/compare/v0.14.0...v0.14.1
[v0.14.0]: https://github.com/brendanzab/cgmath/compare/v0.13.1...v0.14.0
[v0.13.1]: https://github.com/brendanzab/cgmath/compare/v0.13.0...v0.13.1
[v0.12.0]: https://github.com/brendanzab/cgmath/compare/v0.12.0...v0.13.0
[v0.12.0]: https://github.com/brendanzab/cgmath/compare/v0.11.0...v0.12.0
[v0.11.0]: https://github.com/brendanzab/cgmath/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/brendanzab/cgmath/compare/v0.9.1...v0.10.0
[v0.9.1]: https://github.com/brendanzab/cgmath/compare/v0.9.0...v0.9.1
[v0.9.0]: https://github.com/brendanzab/cgmath/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/brendanzab/cgmath/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/brendanzab/cgmath/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/brendanzab/cgmath/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/brendanzab/cgmath/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/brendanzab/cgmath/compare/v0.3.1...v0.4.0
[v0.3.1]: https://github.com/brendanzab/cgmath/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/brendanzab/cgmath/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/brendanzab/cgmath/compare/v0.1.6...v0.2.0
[v0.1.6]: https://github.com/brendanzab/cgmath/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/brendanzab/cgmath/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/brendanzab/cgmath/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/brendanzab/cgmath/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/brendanzab/cgmath/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/brendanzab/cgmath/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/brendanzab/cgmath/compare/v0.0.8...v0.1.0
[v0.0.8]: https://github.com/brendanzab/cgmath/compare/v0.0.7...v0.0.8
[v0.0.7]: https://github.com/brendanzab/cgmath/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/brendanzab/cgmath/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/brendanzab/cgmath/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/brendanzab/cgmath/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/brendanzab/cgmath/compare/v0.0.1...v0.0.3
