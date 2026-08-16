# swizzle-off fixture

This crate is **intentionally broken**. It is not a workspace member and is
not run by CI -- if you build it as part of a recursive `cargo build` sweep
over `compat/fixtures/*` and hit a failure here, that failure is the point,
not a bug to fix.

## What it proves

`Vector2::xy()` (and every other swizzle method) must not exist on the
public API when the `swizzle` feature is off. This crate depends on
`cgmath-next` without that feature and calls `.xy()`.

## Expected result

```
cargo build
```

run from this directory must fail with:

```
error[E0599]: no method named `xy` found for struct `Vector2<S>` in the current scope
```

Positive control -- confirms the same source builds cleanly once the
feature the test is checking for is actually on:

```
cargo build --features cgmath-next/swizzle
```

If either command's result flips (the first succeeds, or the second
fails), that is a real regression in the `swizzle` feature gate --
see `docs/compatibility.md`'s swizzle section for the full ON/OFF
rustdoc-JSON API diff this fixture complements.

This fixture is not wired into CI (see `docs/compatibility.md`'s
"Reverse-dependency fixtures" section) -- it was verified manually
when written and is not re-checked automatically on every change.
