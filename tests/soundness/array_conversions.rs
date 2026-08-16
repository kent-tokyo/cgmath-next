// Regression tests for UNSAFE-001 (docs/unsafe-audit.md): the
// `mem::transmute`-based `AsRef`/`AsMut`/`From<&[S; N]>`/`From<&mut [S; N]>`
// conversions between each type and its fixed-size-array representation.
//
// Unlike UNSAFE-002 (tuple layout, unspecified by the language), these
// transmutes rely on `#[repr(C)]` layout guarantees the language reference
// does make, so no runtime guard was added for this category (see
// docs/unsafe-audit.md UNSAFE-001, "audited sound"). What these tests add
// beyond the existing (non-Miri) `tests/vector.rs`/`tests/point.rs`/
// `tests/matrix.rs` array round-trip assertions is Miri coverage: run
// under `cargo +nightly miri test --test soundness`, Miri's Stacked
// Borrows / aliasing checker exercises every transmuted reference this
// file creates, not just the values it computes.
//
// Every type below is checked two ways:
// - struct -> array view (`as_ref`/`as_mut`): values match the struct's
//   fields in field order, and mutating through the array view (returned
//   by `as_mut`) writes back into the original struct.
// - array -> struct view (`From<&[S; N]>`/`From<&mut [S; N]>`): values
//   match the array's elements, and mutating through the struct view
//   (returned by `From<&mut _>`) writes back into the original array.
// Both directions alias the same memory through a transmuted reference --
// exactly what Miri's aliasing model can catch and plain `cargo test` can't.

use cgmath::{
    Matrix2, Matrix3, Matrix4, Point1, Point2, Point3, Quaternion, Vector1, Vector2, Vector3,
    Vector4,
};

#[test]
fn vector1_array_view_aliases_correctly() {
    let mut v = Vector1::new(1.0f64);
    assert_eq!(*AsRef::<[f64; 1]>::as_ref(&v), [1.0]);
    AsMut::<[f64; 1]>::as_mut(&mut v)[0] = 9.0;
    assert_eq!(v.x, 9.0);

    let mut a = [1.0f64];
    assert_eq!(<&Vector1<f64>>::from(&a).x, 1.0);
    <&mut Vector1<f64>>::from(&mut a).x = 9.0;
    assert_eq!(a, [9.0]);
}

#[test]
fn vector2_array_view_aliases_correctly() {
    let mut v = Vector2::new(1.0f64, 2.0);
    assert_eq!(*AsRef::<[f64; 2]>::as_ref(&v), [1.0, 2.0]);
    let arr = AsMut::<[f64; 2]>::as_mut(&mut v);
    arr[0] = 9.0;
    arr[1] = 8.0;
    assert_eq!((v.x, v.y), (9.0, 8.0));

    let mut a = [1.0f64, 2.0];
    let sref = <&Vector2<f64>>::from(&a);
    assert_eq!((sref.x, sref.y), (1.0, 2.0));
    let smut = <&mut Vector2<f64>>::from(&mut a);
    smut.x = 9.0;
    smut.y = 8.0;
    assert_eq!(a, [9.0, 8.0]);
}

#[test]
fn vector3_array_view_aliases_correctly() {
    let mut v = Vector3::new(1.0f64, 2.0, 3.0);
    assert_eq!(*AsRef::<[f64; 3]>::as_ref(&v), [1.0, 2.0, 3.0]);
    let arr = AsMut::<[f64; 3]>::as_mut(&mut v);
    arr[0] = 9.0;
    arr[1] = 8.0;
    arr[2] = 7.0;
    assert_eq!((v.x, v.y, v.z), (9.0, 8.0, 7.0));

    let mut a = [1.0f64, 2.0, 3.0];
    let sref = <&Vector3<f64>>::from(&a);
    assert_eq!((sref.x, sref.y, sref.z), (1.0, 2.0, 3.0));
    let smut = <&mut Vector3<f64>>::from(&mut a);
    smut.x = 9.0;
    smut.y = 8.0;
    smut.z = 7.0;
    assert_eq!(a, [9.0, 8.0, 7.0]);
}

#[test]
fn vector4_array_view_aliases_correctly() {
    let mut v = Vector4::new(1.0f64, 2.0, 3.0, 4.0);
    assert_eq!(*AsRef::<[f64; 4]>::as_ref(&v), [1.0, 2.0, 3.0, 4.0]);
    let arr = AsMut::<[f64; 4]>::as_mut(&mut v);
    arr[0] = 9.0;
    arr[1] = 8.0;
    arr[2] = 7.0;
    arr[3] = 6.0;
    assert_eq!((v.x, v.y, v.z, v.w), (9.0, 8.0, 7.0, 6.0));

    let mut a = [1.0f64, 2.0, 3.0, 4.0];
    let sref = <&Vector4<f64>>::from(&a);
    assert_eq!((sref.x, sref.y, sref.z, sref.w), (1.0, 2.0, 3.0, 4.0));
    let smut = <&mut Vector4<f64>>::from(&mut a);
    smut.x = 9.0;
    smut.y = 8.0;
    smut.z = 7.0;
    smut.w = 6.0;
    assert_eq!(a, [9.0, 8.0, 7.0, 6.0]);
}

#[test]
fn point1_array_view_aliases_correctly() {
    let mut p = Point1::new(1.0f64);
    assert_eq!(*AsRef::<[f64; 1]>::as_ref(&p), [1.0]);
    AsMut::<[f64; 1]>::as_mut(&mut p)[0] = 9.0;
    assert_eq!(p.x, 9.0);

    let mut a = [1.0f64];
    assert_eq!(<&Point1<f64>>::from(&a).x, 1.0);
    <&mut Point1<f64>>::from(&mut a).x = 9.0;
    assert_eq!(a, [9.0]);
}

#[test]
fn point2_array_view_aliases_correctly() {
    let mut p = Point2::new(1.0f64, 2.0);
    assert_eq!(*AsRef::<[f64; 2]>::as_ref(&p), [1.0, 2.0]);
    let arr = AsMut::<[f64; 2]>::as_mut(&mut p);
    arr[0] = 9.0;
    arr[1] = 8.0;
    assert_eq!((p.x, p.y), (9.0, 8.0));

    let mut a = [1.0f64, 2.0];
    let sref = <&Point2<f64>>::from(&a);
    assert_eq!((sref.x, sref.y), (1.0, 2.0));
    let smut = <&mut Point2<f64>>::from(&mut a);
    smut.x = 9.0;
    smut.y = 8.0;
    assert_eq!(a, [9.0, 8.0]);
}

#[test]
fn point3_array_view_aliases_correctly() {
    let mut p = Point3::new(1.0f64, 2.0, 3.0);
    assert_eq!(*AsRef::<[f64; 3]>::as_ref(&p), [1.0, 2.0, 3.0]);
    let arr = AsMut::<[f64; 3]>::as_mut(&mut p);
    arr[0] = 9.0;
    arr[1] = 8.0;
    arr[2] = 7.0;
    assert_eq!((p.x, p.y, p.z), (9.0, 8.0, 7.0));

    let mut a = [1.0f64, 2.0, 3.0];
    let sref = <&Point3<f64>>::from(&a);
    assert_eq!((sref.x, sref.y, sref.z), (1.0, 2.0, 3.0));
    let smut = <&mut Point3<f64>>::from(&mut a);
    smut.x = 9.0;
    smut.y = 8.0;
    smut.z = 7.0;
    assert_eq!(a, [9.0, 8.0, 7.0]);
}

#[test]
fn quaternion_array_view_aliases_correctly() {
    // Quaternion::new(w, xi, yj, zk); the array form is [xi, yj, zk, w]
    // (see `impl From<[S; 4]> for Quaternion<S>`, `src/quaternion.rs`).
    let mut q = Quaternion::new(4.0f64, 1.0, 2.0, 3.0);
    assert_eq!(*AsRef::<[f64; 4]>::as_ref(&q), [1.0, 2.0, 3.0, 4.0]);
    let arr = AsMut::<[f64; 4]>::as_mut(&mut q);
    arr[0] = 9.0;
    arr[1] = 8.0;
    arr[2] = 7.0;
    arr[3] = 6.0;
    assert_eq!((q.v.x, q.v.y, q.v.z, q.s), (9.0, 8.0, 7.0, 6.0));

    let mut a = [1.0f64, 2.0, 3.0, 4.0];
    let sref = <&Quaternion<f64>>::from(&a);
    assert_eq!((sref.v.x, sref.v.y, sref.v.z, sref.s), (1.0, 2.0, 3.0, 4.0));
    let smut = <&mut Quaternion<f64>>::from(&mut a);
    smut.v.x = 9.0;
    smut.v.y = 8.0;
    smut.v.z = 7.0;
    smut.s = 6.0;
    assert_eq!(a, [9.0, 8.0, 7.0, 6.0]);
}

#[rustfmt::skip]
#[test]
fn matrix2_nested_array_view_aliases_correctly() {
    let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
    assert_eq!(*AsRef::<[[f64; 2]; 2]>::as_ref(&m), [[1.0, 2.0], [3.0, 4.0]]);
    let arr = AsMut::<[[f64; 2]; 2]>::as_mut(&mut m);
    arr[0] = [9.0, 8.0];
    arr[1] = [7.0, 6.0];
    assert_eq!(m, Matrix2::new(9.0, 8.0, 7.0, 6.0));

    let mut a = [[1.0f64, 2.0], [3.0, 4.0]];
    let sref = <&Matrix2<f64>>::from(&a);
    assert_eq!(*sref, Matrix2::new(1.0, 2.0, 3.0, 4.0));
    let smut = <&mut Matrix2<f64>>::from(&mut a);
    *smut = Matrix2::new(9.0, 8.0, 7.0, 6.0);
    assert_eq!(a, [[9.0, 8.0], [7.0, 6.0]]);
}

#[rustfmt::skip]
#[test]
fn matrix2_flat_array_view_aliases_correctly() {
    let mut m = Matrix2::new(1.0f64, 2.0, 3.0, 4.0);
    assert_eq!(*AsRef::<[f64; 4]>::as_ref(&m), [1.0, 2.0, 3.0, 4.0]);
    let arr = AsMut::<[f64; 4]>::as_mut(&mut m);
    for (i, x) in arr.iter_mut().enumerate() {
        *x = 9.0 - i as f64;
    }
    assert_eq!(m, Matrix2::new(9.0, 8.0, 7.0, 6.0));

    let mut a = [1.0f64, 2.0, 3.0, 4.0];
    let sref: &Matrix2<f64> = From::from(&a);
    assert_eq!(*sref, Matrix2::new(1.0, 2.0, 3.0, 4.0));
    let smut: &mut Matrix2<f64> = From::from(&mut a);
    *smut = Matrix2::new(9.0, 8.0, 7.0, 6.0);
    assert_eq!(a, [9.0, 8.0, 7.0, 6.0]);
}

#[rustfmt::skip]
#[test]
fn matrix3_nested_array_view_aliases_correctly() {
    let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    assert_eq!(
        *AsRef::<[[f64; 3]; 3]>::as_ref(&m),
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]
    );
    let arr = AsMut::<[[f64; 3]; 3]>::as_mut(&mut m);
    arr[0] = [9.0, 8.0, 7.0];
    arr[1] = [6.0, 5.0, 4.0];
    arr[2] = [3.0, 2.0, 1.0];
    assert_eq!(m, Matrix3::new(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0));

    let mut a = [[1.0f64, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let sref = <&Matrix3<f64>>::from(&a);
    assert_eq!(*sref, Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0));
    let smut = <&mut Matrix3<f64>>::from(&mut a);
    *smut = Matrix3::new(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
    assert_eq!(a, [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);
}

#[rustfmt::skip]
#[test]
fn matrix3_flat_array_view_aliases_correctly() {
    let mut m = Matrix3::new(1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    assert_eq!(
        *AsRef::<[f64; 9]>::as_ref(&m),
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    );
    let arr = AsMut::<[f64; 9]>::as_mut(&mut m);
    for (i, x) in arr.iter_mut().enumerate() {
        *x = 9.0 - i as f64;
    }
    assert_eq!(m, Matrix3::new(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0));

    let mut a = [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let sref: &Matrix3<f64> = From::from(&a);
    assert_eq!(*sref, Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0));
    let smut: &mut Matrix3<f64> = From::from(&mut a);
    *smut = Matrix3::new(9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
    assert_eq!(a, [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
}

#[rustfmt::skip]
#[test]
fn matrix4_nested_array_view_aliases_correctly() {
    let mut m = Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    assert_eq!(
        *AsRef::<[[f64; 4]; 4]>::as_ref(&m),
        [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]
    );
    let arr = AsMut::<[[f64; 4]; 4]>::as_mut(&mut m);
    for (i, col) in arr.iter_mut().enumerate() {
        for (j, x) in col.iter_mut().enumerate() {
            *x = (16 - (i * 4 + j)) as f64;
        }
    }
    assert_eq!(m, Matrix4::new(
        16.0, 15.0, 14.0, 13.0,
        12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    ));

    let mut a = [
        [1.0f64, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ];
    let sref = <&Matrix4<f64>>::from(&a);
    assert_eq!(*sref, Matrix4::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    ));
    let smut = <&mut Matrix4<f64>>::from(&mut a);
    *smut = Matrix4::new(
        16.0, 15.0, 14.0, 13.0,
        12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    );
    assert_eq!(a, [
        [16.0, 15.0, 14.0, 13.0],
        [12.0, 11.0, 10.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);
}

#[rustfmt::skip]
#[test]
fn matrix4_flat_array_view_aliases_correctly() {
    let mut m = Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    assert_eq!(
        *AsRef::<[f64; 16]>::as_ref(&m),
        [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]
    );
    let arr = AsMut::<[f64; 16]>::as_mut(&mut m);
    for (i, x) in arr.iter_mut().enumerate() {
        *x = (16 - i) as f64;
    }
    assert_eq!(m, Matrix4::new(
        16.0, 15.0, 14.0, 13.0,
        12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    ));

    let mut a = [
        1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let sref: &Matrix4<f64> = From::from(&a);
    assert_eq!(*sref, Matrix4::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    ));
    let smut: &mut Matrix4<f64> = From::from(&mut a);
    *smut = Matrix4::new(
        16.0, 15.0, 14.0, 13.0,
        12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    );
    assert_eq!(a, [16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
}
