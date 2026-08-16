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
