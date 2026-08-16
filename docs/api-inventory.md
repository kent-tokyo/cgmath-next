# Public API inventory (AGENTS.md section 6/11.1)

## Method

`cargo-public-api` was not installed (no network-restricted `cargo install`
attempted); rustdoc's own unstable JSON output is the zero-install
equivalent and was used directly:

```bash
cargo +nightly rustdoc --all-features -- -Zunstable-options --output-format json
```

Run once against the pristine, unmodified import
(`_extract/cgmath-0.18.0/`, i.e. the exact `chore: import cgmath 0.18.0
release source` commit's content) and once against this repository's
current `cgmath-next` (after the package rename and the swap_columns/
swap_elements fix). Both produce `target/doc/cgmath.json` -- same filename
in both cases, because `[lib] name = "cgmath"` is unchanged (see
`docs/compatibility.md`), so each was copied out and renamed immediately
after generation to avoid one overwriting the other.

`format_version` in both JSON files: `57`. Toolchain: the same pinned
`nightly-aarch64-apple-darwin` used for Miri elsewhere in this session
(rustdoc JSON output is nightly-only, `-Zunstable-options`).

The raw JSON (~4.3 MB each) isn't checked in -- it's fully regeneratable
with the command above and mostly consists of every transitively-visible
external dependency item (std, approx, num-traits, serde, etc.), not just
`cgmath`'s own API. What's committed
(`compat/api-inventory/api-paths-0.18.0.txt`) is the `paths` map --
crate-relative `kind<TAB>path` for every item referenced anywhere in the
crate graph -- extracted, deduplicated, and sorted, since raw rustdoc JSON
item IDs are allocation-order-dependent and not meaningful to diff
directly.

## Result

**The path diff is empty.** `diff <(paths from pristine 0.18.0) <(paths
from cgmath-next)` produces zero lines. Both sides have exactly 2867
unique `kind<TAB>path` entries, and of those, exactly 56 are in the
crate's own `cgmath.*` namespace (the rest are transitively-referenced
external items) -- both counts identical between pristine and
`cgmath-next`.

This is the expected result, not a coincidence: no public item was added,
removed, renamed, or moved in this session, and keeping `[lib] name =
"cgmath"` (rather than letting it default to `cgmath_next`) means even the
crate's *own* self-referential path prefix in the JSON (`cgmath.matrix.
Matrix2`, etc.) stayed identical rather than becoming `cgmath_next.matrix.
Matrix2`.

As a second, independent check specifically on the functions this session
touched: the `index` entries (not just `paths`) for every `swap_columns`/
`swap_elements`/`swap_rows` method across `Array`, `Matrix2`, `Matrix3`,
`Matrix4` (13 total) were extracted and their `sig` (parameter types,
return type, generics) compared directly -- byte-identical between
pristine and `cgmath-next`. Only the function *bodies* changed (unsafe
`ptr::swap` replaced with a safe read-into-temp-then-write sequence, see
the fix commit); no signature touched.

## What this diff does *not* cover

- Trait *implementations* (which types implement which traits) aren't
  captured by the `paths` map on its own -- only item existence/kind/path.
  Not separately diffed this session; nothing in the fix commit adds,
  removes, or changes a trait impl, so this is asserted from the diff
  scope (only 2 files touched, both function bodies) rather than a second
  machine-verified pass.
- Doc comments, deprecation attributes, and `cfg` gating aren't part of
  the `paths` map either. None were touched by the fix commit.
- This diff is pristine-0.18.0 vs. current `cgmath-next` -- i.e. it covers
  every commit in this session, not just the swap fix in isolation.

## Acceptable vs. not, per AGENTS.md 11.1

Since the diff is empty, none of the "not acceptable" categories (type
removal, method removal/rename, parameter reordering, return type change,
bound strengthening, field visibility change, enum variant removal,
operator meaning change, layout change) apply. Nothing to report here.
