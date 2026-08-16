// Fixture: dependency rename per AGENTS.md section 4.
//
//     cgmath = { package = "cgmath-next", path = "../../.." }
//
// This is the exact form documented for real users:
//
//     cgmath = { package = "cgmath-next", version = "0.18.1" }
//
// Existing code that does `use cgmath::...` must compile unmodified.
use cgmath::{Matrix4, Quaternion, Vector3};

#[test]
fn unmodified_upstream_import_path_compiles() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);
    let _m = Matrix4::<f32>::from_scale(1.0);
    let _q = Quaternion::<f32>::from_sv(1.0, Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(v.x, 1.0);
}
