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
