# Reverse-dependency migration fixtures (AGENTS.md §14)

3 real crates.io reverse-dependencies of `cgmath` (of 280 total, filtered
to `normal`-kind dependencies on `^0.18`/`^0.18.0`), spanning graphics,
CAD/geometry, and GPU-layout/serialization categories, downloaded as their
published tarballs (same method as the crate's own provenance import) and
tested both against real `cgmath = "0.18.0"` (baseline) and against
`cgmath-next` via `cgmath = { package = "cgmath-next", path = "../../../.." }`.

This meets AGENTS.md §20's "3件以上" alpha gate. It does not yet meet the
5-crate stable gate.

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
