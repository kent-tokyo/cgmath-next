// Fixture: same rename form as rename-dep, but inside a Cargo workspace,
// to confirm behavior does not change inside vs. outside a workspace
// (AGENTS.md section 4, condition 5).
use cgmath::Vector3;

#[test]
fn rename_works_inside_workspace() {
    let v = Vector3::new(1.0f32, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
}
