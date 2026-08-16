// Fixture: cgmath-next used standalone, no dependency rename.
// AGENTS.md section 4 requires this to work as normal usage. Because the
// package's [lib] name is "cgmath" (kept from upstream, see Cargo.toml),
// the import path is `cgmath::...`, not `cgmath_next::...`, even though
// the Cargo.toml dependency key is `cgmath-next`.
use cgmath::Vector3;

#[test]
fn standalone_import_path_is_cgmath() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
}
