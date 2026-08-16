# Reverse-dependency migration fixtures (AGENTS.md §14)

5 real crates.io reverse-dependencies of `cgmath` (of 280 total, filtered
to `normal`-kind dependencies on `^0.18`/`^0.18.0`), spanning graphics,
CAD/geometry, GPU-layout/serialization, multi-backend-abstraction, and
3D-rendering-engine categories, downloaded as their published tarballs
(same method as the crate's own provenance import) and tested both
against real `cgmath = "0.18.0"` (baseline) and against `cgmath-next` via
`cgmath = { package = "cgmath-next", path = "../../../.." }`.

This meets both AGENTS.md §20's "3件以上" alpha gate and the "5件以上"
stable gate.

**Not vendored into this repository.** The downloaded sources (and this
analysis) were produced from the exact tarballs below; the source itself
isn't checked in here to avoid bundling ~300KB of third-party code with
inconsistent LICENSE-file packaging (e.g. `truck-base`'s published tarball
doesn't include a LICENSE file at all, only the `license = "Apache-2.0"`
SPDX field in `Cargo.toml` -- likely because it's a workspace member and
the license file lives at the workspace root upstream). To reproduce:

```bash
UA="cgmath-next-research/0.1 (<your email>)"
curl -sL -A "$UA" "https://crates.io/api/v1/crates/<name>/<version>/download" -o x.crate
tar xzf x.crate --strip-components=1 -C <dest>
```

| Crate | Version | sha256 (tarball) |
|---|---|---|
| `arcball` | 1.1.0 | `5b32e1408f89d00ea90c028c97f928361b9b2dfb7ce122819704726bb6862235` |
| `crevice` | 0.20.1 | `b3f5b73a35775798e5a941a98d3eda7dd6ac6ba4715bd3cce8fef6bdf1a74c91` |
| `truck-base` | 0.5.0 | `4c279de9e92e5dc20a188deb0bb9a4bd421a6a185e57e03e19025e84a36c5a05` |
| `vector-traits` | 0.6.2 | `f228b57b00a0cf34733af60bf2f071c90bf0f6432499c1f9cfd16d13bb4225a5` |
| `three-d` | 0.19.0 | `e73bbcfd30f69623bed55b90bcc084d316fab03a490c2087935c2aa287bd6995` |

---

```
crate name: arcball
original version: 1.1.0
features: default
original result: pass (cargo check, exit 0)
cgmath-next result: pass (cargo check, exit 0)
source changes required: none
failure reason: n/a
```

No `#[test]` functions exist in `arcball`'s own source. `cargo test`
fails to *build* due to an unrelated `winit`/`cocoa` dev-dependency
(pulled in via the `glium` dev-dependency for its example) that doesn't
support the current macOS SDK -- this reproduces identically regardless
of which `cgmath` is used, since it's entirely inside `cocoa`'s own
Objective-C binding code, unrelated to `cgmath`/`cgmath-next`. Not
counted as a cgmath-next failure.

---

```
crate name: crevice
original version: 0.20.1
features: cgmath (optional feature, `cargo test --features cgmath`)
original result: pass (2 + 2 + 9 = 13 tests, 0 failed)
cgmath-next result: pass (2 + 2 + 9 = 13 tests, 0 failed)
source changes required: none
failure reason: n/a
```

Clean pass, no caveats. `crevice` also depends on `mint` directly
(independent of its `cgmath` feature), so this fixture incidentally
touches the `mint` interop path too, though not through `cgmath-next`'s
own `mint` feature specifically.

---

```
crate name: truck-base
original version: 0.5.0
features: default (cgmath is a required, non-optional dependency here, with the `serde` feature enabled)
original result: pass (cargo check; test binaries not separately counted at baseline)
cgmath-next result: pass after 4 one-line source changes (5 + 16 + 34 = 55 tests, 0 failed)
source changes required: yes -- see below
failure reason: see below (not a cgmath-next compatibility gap; see classification)
```

**This is the interesting one.** `truck-base` also depends on
`matext4cgmath` (an unofficial third-party `cgmath` extension crate,
itself pinned to real `cgmath = "0.18.0"`), which does `pub use cgmath;`
-- publicly re-exporting the whole `cgmath` crate under its own name.
`truck-base`'s own `src/cgmath64.rs` does `pub use cgmath::prelude::*;`
and a macro-generated `pub type $typename = cgmath::$typename<f64>;`,
relying on the *implicit* extern-prelude `cgmath` (which, after our
rename, is `cgmath-next`) -- but it also does `pub use matext4cgmath::*;`
in the same file, which brings `matext4cgmath`'s re-exported `cgmath`
(real, unrenamed `cgmath` 0.18.0, since `matext4cgmath` itself was not
part of this rename) into the very same scope under the very same name.

Result: `` `cgmath` is ambiguous `` (E0659) at 3 call sites, because
`truck-base`'s own crate root now has two different crates both visible
as plain `cgmath` -- one via its own (renamed) extern dependency, one via
`matext4cgmath`'s glob-reexport of the *original* crate.

**Classification (per AGENTS.md §14):** neither a `cgmath-next`
compatibility gap nor `truck-base`'s dependence on private/undocumented
`cgmath` internals. It's a third category: a **multi-crate rename
propagation gap**. Renaming only the directly-depended-on `cgmath` in
`truck-base`'s own `Cargo.toml` is not sufficient when a *transitive*
dependency (`matext4cgmath`) also depends on and publicly re-exports
`cgmath` under its own, un-renamed identity. A real `truck-base`
maintainer wanting to migrate would need `matext4cgmath` to migrate too
(or vendor/fork it), not just change one line in their own `Cargo.toml`.

**The fix applied to this fixture** (4 one-line changes, all in
`src/cgmath64.rs` and `src/tolerance.rs`, none in `cgmath-next` itself):
qualify every unqualified `cgmath::` reference in `truck-base`'s own
source that collides with `matext4cgmath`'s re-export as `::cgmath::`
(absolute path, unambiguously truck-base's own direct dependency). Two
rounds were needed: the first round fixed the 3 compile errors, but left
a *silent* type-identity split -- the macro's `cgmath::$typename<f64>`
line, inside the same file, still resolved to `matext4cgmath`'s
re-exported real `cgmath` (since it was no longer ambiguous once the
*other* uses were qualified away), so all of `truck-base`'s own
`Vector2`/`Vector3`/`Matrix3`/etc. f64-specialized type aliases were
silently bound to real-`cgmath` types instead of `cgmath-next` types.
This surfaced as `FromIterator`/trait-not-implemented errors in 33/34
doctests (`BoundingBox<V>` impls exist for `cgmath-next`'s `Vector2`, not
real `cgmath`'s -- two nominally-identical but crate-distinct types).
Fixing the macro's `cgmath::$typename` to `::cgmath::$typename` as well
resolved everything: 5 lib tests + 16 integration tests + 34 doctests,
all passing.

This is recorded in detail because it's a genuinely useful data point for
`docs/migration.md`: **crates that depend on both `cgmath` and a `cgmath`-
extension crate need the extension crate to migrate too, and a partial
local fix can silently succeed at compiling while leaving a live type
split -- worth calling out explicitly for anyone attempting this
migration pattern.**

The complete diff applied (`Cargo.toml`'s dependency rename plus 4 source
lines, all in `truck-base`'s own code, none in `cgmath-next`):

```diff
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -dependencies.cgmath
 [dependencies.cgmath]
-version = "0.18.0"
+package = "cgmath-next"
+path = "../../../.."
 features = ["serde"]

--- a/src/cgmath64.rs
+++ b/src/cgmath64.rs
@@
 pub use crate::cgmath_extend_traits::*;
-pub use cgmath::prelude::*;
-pub use cgmath::{frustum, ortho, perspective, Deg, Rad};
+pub use ::cgmath::prelude::*;
+pub use ::cgmath::{frustum, ortho, perspective, Deg, Rad};
 pub use matext4cgmath::*;
 macro_rules! f64_type {
         ($typename: ident) => {
             /// redefinition, scalar = f64
-            pub type $typename = cgmath::$typename<f64>;
+            pub type $typename = ::cgmath::$typename<f64>;
         };

--- a/src/tolerance.rs
+++ b/src/tolerance.rs
@@
 use crate::cgmath64::*;
-use cgmath::AbsDiffEq;
+use ::cgmath::AbsDiffEq;
```

---

```
crate name: vector-traits
original version: 0.6.2
features: cgmath (optional feature, `cargo test --features cgmath --no-default-features`)
original result: pass (13 tests, 0 failed)
cgmath-next result: pass, no changes (13 tests, 0 failed)
source changes required: none
failure reason: n/a
```

`vector-traits` provides trait-based abstractions (`GenericVector2`,
`GenericVector3`, `HasXY`/`HasXYZ`, etc.) implemented identically over
`cgmath`, `glam`, `nalgebra`, and `macaw` types, so a caller can write
generic code once and plug in whichever backend. Like `matext4cgmath` in
the `truck-base` case above, it does `pub use cgmath;` (`src/lib.rs:137`)
-- publicly re-exporting its own `cgmath` dependency. Picked specifically
to stress-test the "extension crate re-exports cgmath" pattern again, this
time with a compile-time check of whether values actually belong to the
same type system, not just "did it compile."

**Migration test (patch `vector-traits`' own `Cargo.toml`, same method as
the other 3 fixtures):** clean pass, 13/13, zero source changes. Unlike
`truck-base`, no ambiguity error at all -- `vector-traits` doesn't glob-
import a second `cgmath` into the same scope as its own re-export, so
there's nothing to collide.

**Compile-time type-identity check (AGENTS.md doesn't require this, but it's
the sharper question "does it compile" doesn't answer):** a separate small
wrapper crate, `type-identity-check`, depends on **both** `cgmath-next`
directly (`package = "cgmath-next"`) **and** the migrated `vector-traits`
copy above (which itself now depends on the exact same `cgmath-next` via
an identical `path`). It constructs a `Vector3<f32>` through its own direct
`cgmath-next` dependency and passes it, with zero conversion, into a
function generic over `vector-traits`' `GenericVector3` trait:

```rust
fn takes_generic_vector3<T: GenericVector3>(v: T) -> T::Scalar {
    v.x() + v.y() + v.z()
}
let v: cgmath::Vector3<f32> = cgmath::Vector3::new(1.0, 2.0, 3.0);
let sum = takes_generic_vector3(v); // no .into(), no wrapper
```

**This compiles and runs (`sum == 6.0`).** Rust trait resolution is exact
nominal-type matching, not structural -- this is not possible unless the
compiler considers the wrapper's own `cgmath::Vector3<f32>` and
`vector-traits`' internal `crate::cgmath::Vector3<f32>` the *identical*
type. Cargo unified the two `path` dependencies (the wrapper's direct one,
`vector-traits`'s transitive one) pointing at the same source into one
crate instance in the build graph, exactly as expected when both point at
the identical path. This empirically confirms `docs/migration.md`'s
"Option 1: get the extension crate to depend on `cgmath-next` too" claim,
which had previously only been argued, not demonstrated.

**Negative control -- same wrapper code, but against the *unmigrated*
`vector-traits` 0.6.2 from crates.io (still on real `cgmath ^0.18`)
instead of the patched copy:** fails to compile, as expected:

```
error[E0277]: the trait bound `cgmath::Vector3<f32>: GenericVector3` is not satisfied
   |
29 |     let sum = takes_generic_vector3(v);
   |               --------------------- ^ the trait `GenericVector3` is not implemented for `cgmath::Vector3<f32>`
   = note: there are multiple different versions of crate `cgmath` in the dependency graph
help: the following other types implement trait `GenericVector3`
   --> .../vector-traits-0.6.2/src/cgmath_impl.rs:533:9
   |
   |         `vector_traits::cgmath::Vector3<f32>`
```

rustc's own diagnostic states the finding better than any prose could:
*"there are multiple different versions of crate `cgmath` in the
dependency graph."* This is not an ambiguity error (`E0659`, the
`truck-base` failure mode) -- it's a plain trait-bound failure, because
without migrating, `vector-traits`' impls target *its own* (real,
unrenamed) `cgmath::Vector3<f32>`, a nominally different type from this
wrapper's `cgmath-next`-backed `Vector3<f32>`, even though both are
structurally identical and both display as "`cgmath::Vector3<f32>`" in
diagnostics.

**Generalizable finding, distinct from `truck-base`'s:** renaming *only
your own* direct `cgmath` dependency does not retroactively make a
third-party generic-trait crate's impls apply to your values, unless that
crate has *also* migrated. This is different in kind from `truck-base`'s
silent-ambiguity trap -- here the compiler refuses outright (E0277), it
never silently compiles with the wrong type. The safe, always-available
fallback when an extension crate hasn't migrated: convert through a
common representation both crates agree on (e.g. `Into<[S; 3]>`, which
both real `cgmath` and `cgmath-next` implement identically per
`docs/api-inventory.md`'s zero-diff result), not a direct trait call.

Not vendored, same reasoning as above; reproduce with:

```bash
UA="cgmath-next-research/0.1 (<your email>)"
curl -sL -A "$UA" "https://crates.io/api/v1/crates/vector-traits/0.6.2/download" -o x.crate
tar xzf x.crate --strip-components=1 -C <dest>
# then patch [dependencies.cgmath] in Cargo.toml to package="cgmath-next", path="<repo root>"
```

---

```
crate name: three-d
original version: 0.19.0
features: --no-default-features (default "window" feature pulls in glutin/winit/wasm-bindgen -- windowing/platform deps unrelated to the cgmath compatibility question this fixture tests; cgmath itself is a required, non-optional, non-gated dependency)
original result: pass (cargo check, exit 0)
cgmath-next result: pass (cargo check, exit 0)
source changes required: none
failure reason: n/a
```

A popular 2D/3D rendering engine (401K downloads on crates.io as of this
writing), picked as the 5th fixture specifically for being a heavier,
real-world consumer outside the graphics/CAD/GPU-layout categories the
first four fixtures already covered. `cgmath` is used throughout for
transform/projection math, not confined to a feature-gated subset.

No `#[test]` functions exist in `three-d`'s own `src/` (confirmed by
grep) -- same situation as the `arcball` fixture above. `cargo test
--no-default-features --lib` runs 0 tests, 0 failed (nothing to run,
not a skip). `cargo test --all-targets` (which would build the example
binaries) was not attempted: three-d's dev-dependencies pull in `tokio`,
`winit`, and windowing libraries needed only for its examples, not for
testing `cgmath`/`cgmath-next` compatibility, and the same
network/platform-dependent-build concern noted for `arcball`'s `cocoa`
dev-dependency applies here too. `cargo check --no-default-features` is
the correct-weight test for what this fixture is actually verifying.

This meets the AGENTS.md §20 stable gate (5 total reverse-dependency
fixtures).

Reproduce with:

```bash
UA="cgmath-next-research/0.1 (<your email>)"
curl -sL -A "$UA" "https://crates.io/api/v1/crates/three-d/0.19.0/download" -o x.crate
tar xzf x.crate --strip-components=1 -C <dest>
# then patch [dependencies.cgmath] in Cargo.toml to package="cgmath-next", path="<repo root>"
# cargo check --no-default-features
```
