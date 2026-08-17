# cgmath-next

[![CI](https://img.shields.io/github/actions/workflow/status/kent-tokyo/cgmath-next/ci.yml?branch=main&label=CI)](https://github.com/kent-tokyo/cgmath-next/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/docsrs/cgmath-next)](https://docs.rs/cgmath-next)
[![crates.io](https://img.shields.io/crates/v/cgmath-next.svg)](https://crates.io/crates/cgmath-next)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/kent-tokyo/cgmath-next/blob/main/LICENSE)

English | [日本語](README_ja.md) | [中文](README_zh.md)

**Still using `cgmath` 0.18?** `cgmath-next` is its actively maintained,
source-compatible successor — same API,
[known soundness issue](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)
fixed.

> Keep the API. Remove the unsoundness.

## Migration

In most cases, migrating is a one-line `Cargo.toml` change and nothing else:

```toml
[dependencies]
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

```rust
use cgmath::{Matrix4, Quaternion, Vector3}; // unchanged
```

This works because `cgmath-next`'s compiled library name is still `cgmath`
(see [`docs/compatibility.md`](docs/compatibility.md) for the fixture-based
verification behind this). See
[`docs/migration.md`](docs/migration.md) for more detail and for cases
where a rename isn't even necessary.

## Why migrate

| | `cgmath` 0.18 | `cgmath-next` 0.18.1 |
|---|---|---|
| API | — | Same |
| Maintenance | [unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0196.html) | Actively maintained |
| Known swap UB ([RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)) | Affected | Fixed |
| Import changes | — | None |
| Public API diff | — | Zero |
| License | Apache-2.0 | Apache-2.0 |

`cgmath-next` is an independently maintained, community successor to
[`cgmath`](https://github.com/rustgd/cgmath) 0.18.0 — not an official
continuation.

## Examples

See [`examples/camera_transform.rs`](examples/camera_transform.rs) for a
minimal, runnable example: building a camera view-projection matrix and
pushing a point through it into clip space and normalized device
coordinates — the same per-vertex pipeline a renderer runs.

```
cargo run --example camera_transform
```

## What this project preserves

* The public API of `cgmath` 0.18.0 as published on crates.io — verified by
  a machine-generated path diff, currently **zero differences**
  (see [`docs/api-inventory.md`](docs/api-inventory.md))
* Numeric results, except where a soundness fix required a change —
  verified by a 9-case differential test suite against real `cgmath`
  0.18.0 with exact (not approximate) equality
  (see [`docs/compatibility.md`](docs/compatibility.md))
* Types and trait implementations, including the `serde`/`mint`/`rand`
  derive impls — unchanged code, but their wire-format output isn't
  independently round-trip-tested against upstream yet (also noted in
  `docs/compatibility.md`)
* The `[lib] name = "cgmath"` the original crate already declared, so
  existing `use cgmath::...` code keeps compiling
* Apache-2.0 licensing and the original copyright notices

## What this project does **not** guarantee

* **Not a "100% memory safe" or "fully audited" claim.** Soundness work is
  ongoing and tracked item-by-item in
  [`docs/unsafe-audit.md`](docs/unsafe-audit.md). One category of existing
  unsafe code (`AsRef`/`AsMut`/`From` conversions to homogeneous tuples,
  e.g. `(f32, f32, f32)`) relies on tuple memory layout that Rust's
  language reference does not formally guarantee — flagged as
  `UNSAFE-002`. This is now **guarded**: a runtime check verifies size,
  alignment, and per-field byte offsets before every such conversion and
  panics instead of transmuting if they don't match (verified via Miri,
  a negative-control test, and release-build disassembly confirming zero
  cost when layout matches, as it does today). This converts a
  hypothetical future layout divergence from silent undefined behavior
  into a loud, immediate panic — it is **not** a language-level soundness
  proof; tuple layout remains officially unspecified, and the only way to
  fully close this out would be removing the reference-returning
  conversions (a public API change, out of scope for this series). See
  `docs/unsafe-audit.md`'s feasibility-study section for the full
  writeup, and [`rustgd/cgmath#538`](https://github.com/rustgd/cgmath/issues/538)
  for independent corroboration that this isn't specific to this fork.
* **No Rust ABI compatibility guarantee.** `#[repr(C)]` layout (`size_of`,
  `align_of`, field offsets) is verified specifically for types that
  declare it, not implied crate-wide.
* **Not a "complete drop-in replacement" for every possible use.** Verified
  compatibility is scoped to what's actually been tested: the upstream test
  suite (unmodified, all passing), a handful of dependency-rename fixtures,
  and 5 real downstream crates (see
  [`compat/fixtures/reverse-deps/RESULTS.md`](compat/fixtures/reverse-deps/RESULTS.md)).
  See [`docs/compatibility.md`](docs/compatibility.md) for exactly what's
  been checked.

## RustSec advisory status

| Advisory | Status |
|---|---|
| [RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html) (soundness: `swap_columns` same-index UB) | **Fixed**, and the fix covers more than the advisory's literal scope — see [`docs/unsafe-audit.md`](docs/unsafe-audit.md) and the project's commit history for `Array::swap_elements` and `Matrix::swap_elements`, which shared the same bug and were not named in the advisory. |
| [RUSTSEC-2026-0196](https://rustsec.org/advisories/RUSTSEC-2026-0196.html) (unmaintained) | Addressed by this project's existence: `cgmath-next` is actively maintained. |

## Relationship to upstream

`cgmath-next` is based on `cgmath` 0.18.0 as published on crates.io, not
upstream's `master` branch (which has diverged with unreleased features and
a dependency bump). See [`docs/provenance.md`](docs/provenance.md) for the
exact source, checksum, and the classification of every upstream commit
since the 0.18.0 tag. See [Release status](#release-status) for what's
published so far.

## Compatibility policy

* Public API removals, renames, signature changes, or trait-bound
  tightening are treated as a release blocker for the 0.18.1 series, not
  something to slip in silently.
* Soundness fixes take priority over strict behavioral preservation when
  the two conflict (e.g. same-index swap becoming an explicit no-op).
* New features and large redesigns are deferred until after the API-parity
  release. See `AGENTS.md` in this repository for the full policy this
  project follows.

## MSRV

See [`docs/msrv.md`](docs/msrv.md) for the measured minimum supported Rust
version and how it was determined.

## Features

Declared vs. implicit (Cargo auto-generates a feature for each optional
dependency) — see [`docs/compatibility.md`](docs/compatibility.md) for the
full breakdown:

| Feature | What it enables |
|---|---|
| `swizzle` | GPU-style swizzle accessors (`v.xyxz()`, etc.) |
| `unstable` | Present for compatibility with 0.18.0; currently gates no reachable code |
| `serde` | `Serialize`/`Deserialize` impls |
| `mint` | Conversions to/from the [`mint`](https://crates.io/crates/mint) interop types |
| `rand` | `Distribution` impls for random generation |

### Swizzling

This library offers an optional feature called
["swizzling"](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics)),
widely familiar to GPU programmers. Enable it with
`--features="swizzle"`.

```rust
let v = Vector3::new(1.0, 2.0, 3.0);
v.xyxz(); // Vector4 { x: 1.0, y: 2.0, z: 1.0, w: 3.0 }
v.zy();   // Vector2 { x: 3.0, y: 2.0 }
```

## Conventions

`cgmath-next` interprets its vectors as column matrices ("column
vectors"), meaning when transforming a vector with a matrix, the matrix
goes on the left. This is reflected in the fact that `cgmath-next`
implements the multiplication operator for `Matrix * Vector`, but not
`Vector * Matrix`. Unchanged from upstream.

## Limitations

`cgmath-next` is _not_ an n-dimensional library and is aimed at computer
graphics applications rather than general linear algebra. It only offers
the 2, 3, and 4 dimensional structures, inherited unchanged from upstream.
Dynamic-dimension matrices, GPU compute, and a `nalgebra`/`glam`-style API
redesign are explicitly out of scope for this release series — see
`AGENTS.md` for the full list.

## Security reporting

See [`SECURITY.md`](SECURITY.md). Soundness issues (safe-Rust-reachable
undefined behavior) are treated as security issues, not ordinary bugs.

## Release status

**`0.18.1` (stable) is published** to crates.io, tagged
([`v0.18.1`](https://github.com/kent-tokyo/cgmath-next/releases/tag/v0.18.1)).
`0.18.1-alpha.1` preceded it as an alpha observation period with no
soundness or compatibility issues reported against it — see
[`docs/release-checklist.md`](docs/release-checklist.md) for the full
gating history, and `docs/unsafe-audit.md`'s `UNSAFE-002` section for
the one known limitation accepted as permanent in this stable series.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## API diff / unsafe audit

* [`docs/api-inventory.md`](docs/api-inventory.md) — machine-generated
  public API comparison against 0.18.0
* [`docs/unsafe-audit.md`](docs/unsafe-audit.md) — every `unsafe` block in
  the crate, its safety invariant, and its verification status
