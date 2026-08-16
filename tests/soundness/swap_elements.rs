// Same root cause as RUSTSEC-2026-0197 (see swap_columns.rs), found by
// grepping the crate for the same `ptr::swap` pattern used in the advisory.
// `Array::swap_elements` (src/structure.rs, used by Vector2/3/4, Point2/3/4,
// and internally by Matrix::swap_rows) and `Matrix::swap_elements` (the
// (col, row) cell-swap in src/matrix.rs) both used
// `ptr::swap(&mut self[i], &mut self[j])` unconditionally. Neither is named
// in the advisory, but both were reachable from safe Rust with attacker- or
// caller-controlled indices, exactly like swap_columns. Not fixing these
// alongside swap_columns would have left the same UB reachable through a
// sibling API.
use cgmath::prelude::*;
use cgmath::{Matrix, Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};

#[test]
fn vector2_swap_elements_same_index_is_noop() {
    for i in 0..2 {
        let mut v = Vector2::new(1.0f64, 2.0);
        let before = v;
        v.swap_elements(i, i);
        assert_eq!(v, before);
    }
}

#[test]
fn vector3_swap_elements_same_index_is_noop() {
    for i in 0..3 {
        let mut v = Vector3::new(1.0f64, 2.0, 3.0);
        let before = v;
        v.swap_elements(i, i);
        assert_eq!(v, before);
    }
}

#[test]
fn vector4_swap_elements_same_index_is_noop() {
    for i in 0..4 {
        let mut v = Vector4::new(1.0f64, 2.0, 3.0, 4.0);
        let before = v;
        v.swap_elements(i, i);
        assert_eq!(v, before);
    }
}

#[test]
fn vector3_swap_elements_distinct_indices_still_swap() {
    let mut v = Vector3::new(1.0f64, 2.0, 3.0);
    v.swap_elements(0, 2);
    assert_eq!(v, Vector3::new(3.0, 2.0, 1.0));
}

#[test]
fn matrix2_swap_elements_same_cell_is_noop() {
    for c in 0..2 {
        for r in 0..2 {
            let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
            let before = m;
            m.swap_elements((c, r), (c, r));
            assert_eq!(m, before);
        }
    }
}

#[test]
fn matrix3_swap_elements_same_cell_is_noop() {
    for c in 0..3 {
        for r in 0..3 {
            let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
            let before = m;
            m.swap_elements((c, r), (c, r));
            assert_eq!(m, before);
        }
    }
}

#[test]
fn matrix4_swap_elements_same_cell_is_noop() {
    for c in 0..4 {
        for r in 0..4 {
            #[rustfmt::skip]
            let mut m = Matrix4::new(
                1.0f64, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 10.0, 11.0, 12.0,
                13.0, 14.0, 15.0, 16.0,
            );
            let before = m;
            m.swap_elements((c, r), (c, r));
            assert_eq!(m, before);
        }
    }
}

#[test]
fn matrix3_swap_elements_distinct_cells_still_swap() {
    let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    m.swap_elements((0, 0), (2, 2));
    assert_eq!(m, Matrix3::new(9.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 1.0));
}

#[test]
fn matrix2_swap_rows_same_index_is_noop() {
    // swap_rows delegates to each column's Array::swap_elements, so it
    // shares the same same-index aliasing hazard.
    for i in 0..2 {
        let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
        let before = m;
        m.swap_rows(i, i);
        assert_eq!(m, before);
    }
}
