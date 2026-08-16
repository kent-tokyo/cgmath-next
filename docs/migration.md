# Migration guide: `cgmath` 0.18.0 -> `cgmath-next`

## The common case: one line

```toml
[dependencies]
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

Nothing else changes. `use cgmath::{Matrix4, Quaternion, Vector3};` and
every other existing import keeps working unmodified, because
`cgmath-next`'s compiled library name is still `cgmath` (`[lib] name =
"cgmath"` in `Cargo.toml`, unchanged from what upstream already
declared). See `docs/compatibility.md` for the fixture-based verification
behind this claim.

You can also depend on it without a rename at all:

```toml
[dependencies]
cgmath-next = "0.18.1"
```

...and `use cgmath::Vector3;` (not `cgmath_next::Vector3`) still works,
for the same `[lib] name` reason. The rename form above is recommended
anyway, since it documents *why* the dependency is there for anyone
reading your `Cargo.toml`.

## If you depend on a `cgmath`-extension crate too

**Read this section if your `Cargo.toml` has both `cgmath` and some other
crate that itself depends on `cgmath`** (a math-extension crate, a
`mint`-bridge crate, a game-engine integration crate, etc.) -- not just a
crate that happens to use `cgmath` internally without exposing it.

Renaming only your own direct `cgmath` dependency is **not always
sufficient**. If the extension crate is still pinned to real `cgmath`
0.18.0 (hasn't migrated to `cgmath-next` itself) *and* it publicly
re-exports `cgmath` (e.g. `pub use cgmath;`, common in small
extension/adapter crates), you can end up with two different crates both
visible under the plain name `cgmath` in your own crate root: one via
your renamed direct dependency, one via the extension crate's
un-renamed re-export. The compiler will report this as an ambiguity
error (`E0659`, "`cgmath` is ambiguous").

This happened in this project's own reverse-dependency testing --
`truck-base` (a real crate, not a hypothetical) hit exactly this with
`matext4cgmath`. Full diagnosis, including a **partial fix that
compiles but silently produces two different, incompatible `Vector`/
`Matrix` types depending on which code path constructed the value** (a
much worse failure mode than a compile error, since it doesn't show up
until you try to use the values together), is recorded in
`compat/fixtures/reverse-deps/RESULTS.md`. **Do not stop at "it compiles
now"** -- if you had to touch anything to resolve an ambiguity like this,
make sure every place in your own code that refers to the bare name
`cgmath` was updated consistently, including inside macros, not just the
specific line the compiler pointed at.

The two real fixes, in order of preference:

1. **Get the extension crate to depend on `cgmath-next` too**, if it's
   actively maintained. This is the clean fix and avoids the ambiguity
   entirely.
2. **If you can't wait for that**, qualify every one of *your own crate's*
   unqualified `cgmath::...` references (including inside your own
   macros, if any) with the absolute path `::cgmath::...`, so they
   unambiguously mean your renamed dependency rather than whatever the
   extension crate re-exports. This does not fix the extension crate
   itself -- values that cross the boundary into the extension crate's
   own API will still be extension-crate's-cgmath types, not
   `cgmath-next` types, and may need explicit conversion.

## What doesn't need to change

* Numeric results, `serde` JSON shape, `mint` conversions, `swizzle`
  behavior -- see `docs/compatibility.md` for what's been verified and
  `docs/api-inventory.md` for the machine-checked public API diff
  (currently empty).
* Workspace vs. non-workspace usage -- no difference, verified by fixture
  (`compat/fixtures/workspace-rename/`).
* Using both old `cgmath` and `cgmath-next` in the same `Cargo.toml` at
  once (e.g. during an incremental migration) -- works, verified by
  fixture (`compat/fixtures/dual-dep/`), as long as you don't also hit
  the extension-crate ambiguity case above.

## If something doesn't compile after the rename

1. Check whether it's the extension-crate ambiguity case above first --
   it's the most likely non-trivial cause.
2. Check `docs/api-inventory.md` and `docs/unsafe-audit.md` -- if the
   difference is a real public API change, it should be listed there. If
   it isn't, that's a `cgmath-next` bug: please report it (see
   `SECURITY.md` if it's a soundness issue, otherwise open a normal
   issue).
3. If you're getting a *value* mismatch rather than a compile error
   (results differ from what `cgmath` 0.18.0 produced), check
   `docs/unsafe-audit.md`'s `UNSAFE-002` entry -- the one known
   unverified risk area is layout-dependent tuple conversions
   (`Into<(S, S, S)>`-style APIs).
