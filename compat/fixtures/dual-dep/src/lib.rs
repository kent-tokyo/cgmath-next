// Fixture: old cgmath 0.18.0 and cgmath-next both depended on at once,
// under distinct local names, to prove they coexist without collision.
#[test]
fn both_crates_coexist_and_agree() {
    let old = cgmath::Vector3::new(1.0f32, 2.0, 3.0);
    let next = cgmath_next::Vector3::new(1.0f32, 2.0, 3.0);
    assert_eq!(old.x, next.x);
    assert_eq!(old.y, next.y);
    assert_eq!(old.z, next.z);
}
