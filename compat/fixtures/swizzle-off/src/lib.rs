// This crate is INTENTIONALLY expected to fail `cargo build`.
//
// The `swizzle` feature is not enabled in Cargo.toml, so `Vector2::xy()`
// (and every other swizzle method) must not exist on the public API.
// `cargo build` here should fail with:
//
//   error[E0599]: no method named `xy` found for struct `Vector2<S>` in the
//   current scope
//
// If this ever compiles successfully, that is the bug: it would mean a
// swizzle method leaked into the public API without the feature enabled.
// See docs/compatibility.md's swizzle section for the full ON/OFF
// rustdoc-JSON API diff this fixture complements.

// [lib] name = "cgmath" in the dependency's own Cargo.toml, so it's
// imported as `cgmath` regardless of the dependency key name above.
use cgmath::Vector2;

#[allow(dead_code)]
fn swizzle_method_must_not_exist_without_the_feature(v: Vector2<f32>) -> Vector2<f32> {
    v.xy()
}
