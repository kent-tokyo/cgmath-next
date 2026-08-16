// Regression tests for RUSTSEC-2026-0197 / rustgd/cgmath#565.
//
// `Matrix{2,3,4}::swap_columns(a, b)` used `ptr::swap(&mut self[a], &mut
// self[b])`. When `a == b` this creates two simultaneous `&mut` borrows of
// the same column through a 100% safe API call, which is a Stacked Borrows
// violation Miri detects as undefined behavior even though the swapped
// values are unchanged. `cargo test` alone cannot see this: the value-level
// assertions below pass on both the buggy and the fixed implementation.
// Run these under `cargo +nightly miri test --test soundness` to catch the
// aliasing violation itself.
use cgmath::{Matrix, Matrix2, Matrix3, Matrix4};

#[test]
fn matrix2_same_index_is_noop_for_every_valid_index() {
    for i in 0..2 {
        let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
        let before = m;
        m.swap_columns(i, i);
        assert_eq!(m, before, "swap_columns({i}, {i}) must not change Matrix2");
    }
}

#[test]
fn matrix3_same_index_is_noop_for_every_valid_index() {
    for i in 0..3 {
        let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let before = m;
        m.swap_columns(i, i);
        assert_eq!(m, before, "swap_columns({i}, {i}) must not change Matrix3");
    }
}

#[test]
fn matrix4_same_index_is_noop_for_every_valid_index() {
    for i in 0..4 {
        #[rustfmt::skip]
        let mut m = Matrix4::new(
            1.0f64, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        );
        let before = m;
        m.swap_columns(i, i);
        assert_eq!(m, before, "swap_columns({i}, {i}) must not change Matrix4");
    }
}

#[test]
fn matrix2_distinct_indices_still_swap() {
    let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
    m.swap_columns(0, 1);
    assert_eq!(m, Matrix2::new(3.0, 4.0, 1.0, 2.0));
}

#[test]
fn matrix3_distinct_indices_still_swap() {
    let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    m.swap_columns(0, 2);
    assert_eq!(
        m,
        Matrix3::new(7.0, 8.0, 9.0, 4.0, 5.0, 6.0, 1.0, 2.0, 3.0)
    );
}

#[test]
fn matrix4_distinct_indices_still_swap() {
    #[rustfmt::skip]
    let mut m = Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    m.swap_columns(1, 3);
    #[rustfmt::skip]
    let expected = Matrix4::new(
        1.0, 2.0, 3.0, 4.0,
        13.0, 14.0, 15.0, 16.0,
        9.0, 10.0, 11.0, 12.0,
        5.0, 6.0, 7.0, 8.0,
    );
    assert_eq!(m, expected);
}

#[test]
fn swap_columns_applied_twice_returns_to_original() {
    #[rustfmt::skip]
    let mut m = Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let before = m;

    m.swap_columns(0, 3);
    m.swap_columns(0, 3);
    assert_eq!(m, before, "swap_columns(a, b) twice must be identity");

    m.swap_columns(2, 2);
    m.swap_columns(2, 2);
    assert_eq!(m, before, "swap_columns(a, a) twice must be identity");
}
