// Differential test harness (AGENTS.md 11.2): real upstream cgmath 0.18.0
// and cgmath-next depended on simultaneously under distinct local names
// (`cgmath` / `cgmath_next`), same inputs fed to both, outputs compared.
//
// Since cgmath-next's only functional change vs. 0.18.0 is the
// swap_columns/swap_elements soundness fix (which does not change output
// values for any of the operations below -- see the fix commit and
// tests/soundness/), every comparison here uses exact `==`, not an approx
// tolerance. An exact-equality differential test is strictly stronger than
// an approx one and is only possible because the two crates currently
// share (almost) identical source.
// Both crates' preludes export same-named traits (InnerSpace, Matrix,
// SquareMatrix, ...); glob-importing both at once makes method resolution
// ambiguous in a way that silently fails as "method not found" rather than
// erroring on the ambiguity itself. `as _` imports the trait's methods into
// scope without binding a name, so both crates' identically-named traits
// can coexist.
use cgmath::prelude::EuclideanSpace as _;
use cgmath::prelude::InnerSpace as _;
use cgmath::prelude::Matrix as _;
use cgmath::prelude::Rotation as _;
use cgmath::prelude::SquareMatrix as _;
use cgmath_next::prelude::EuclideanSpace as _;
use cgmath_next::prelude::InnerSpace as _;
use cgmath_next::prelude::Matrix as _;
use cgmath_next::prelude::Rotation as _;
use cgmath_next::prelude::SquareMatrix as _;

fn v3_old(x: f64, y: f64, z: f64) -> cgmath::Vector3<f64> {
    cgmath::Vector3::new(x, y, z)
}
fn v3_next(x: f64, y: f64, z: f64) -> cgmath_next::Vector3<f64> {
    cgmath_next::Vector3::new(x, y, z)
}

fn assert_v3_eq(old: cgmath::Vector3<f64>, next: cgmath_next::Vector3<f64>) {
    assert_eq!(old.x, next.x);
    assert_eq!(old.y, next.y);
    assert_eq!(old.z, next.z);
}

#[test]
fn vector_add_sub_mul_div() {
    let (a_o, b_o) = (v3_old(1.0, 2.0, 3.0), v3_old(4.0, -5.0, 6.5));
    let (a_n, b_n) = (v3_next(1.0, 2.0, 3.0), v3_next(4.0, -5.0, 6.5));

    assert_v3_eq(a_o + b_o, a_n + b_n);
    assert_v3_eq(a_o - b_o, a_n - b_n);
    assert_v3_eq(a_o * 2.5, a_n * 2.5);
    assert_v3_eq(a_o / 2.5, a_n / 2.5);
}

#[test]
fn vector_dot_cross_magnitude_normalize() {
    let (a_o, b_o) = (v3_old(1.0, 2.0, 3.0), v3_old(4.0, -5.0, 6.5));
    let (a_n, b_n) = (v3_next(1.0, 2.0, 3.0), v3_next(4.0, -5.0, 6.5));

    assert_eq!(a_o.dot(b_o), a_n.dot(b_n));
    assert_v3_eq(a_o.cross(b_o), a_n.cross(b_n));
    assert_eq!(a_o.magnitude(), a_n.magnitude());
    assert_v3_eq(a_o.normalize(), a_n.normalize());
}

#[test]
fn matrix_add_sub_mul_transpose_determinant_invert() {
    #[rustfmt::skip]
    let m_o = cgmath::Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        0.0, 1.0, 4.0, 6.0,
        5.0, 6.0, 0.0, 2.0,
        1.0, 0.0, 0.0, 1.0,
    );
    #[rustfmt::skip]
    let m_n = cgmath_next::Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        0.0, 1.0, 4.0, 6.0,
        5.0, 6.0, 0.0, 2.0,
        1.0, 0.0, 0.0, 1.0,
    );
    #[rustfmt::skip]
    let n_o = cgmath::Matrix4::new(
        2.0f64, 0.0, 1.0, 0.0,
        1.0, 3.0, 0.0, 1.0,
        0.0, 1.0, 2.0, 0.0,
        1.0, 0.0, 1.0, 1.0,
    );
    #[rustfmt::skip]
    let n_n = cgmath_next::Matrix4::new(
        2.0f64, 0.0, 1.0, 0.0,
        1.0, 3.0, 0.0, 1.0,
        0.0, 1.0, 2.0, 0.0,
        1.0, 0.0, 1.0, 1.0,
    );

    let sum_o: [[f64; 4]; 4] = (m_o + n_o).into();
    let sum_n: [[f64; 4]; 4] = (m_n + n_n).into();
    assert_eq!(sum_o, sum_n);

    let diff_o: [[f64; 4]; 4] = (m_o - n_o).into();
    let diff_n: [[f64; 4]; 4] = (m_n - n_n).into();
    assert_eq!(diff_o, diff_n);

    let prod_o: [[f64; 4]; 4] = (m_o * n_o).into();
    let prod_n: [[f64; 4]; 4] = (m_n * n_n).into();
    assert_eq!(prod_o, prod_n);

    let t_o: [[f64; 4]; 4] = m_o.transpose().into();
    let t_n: [[f64; 4]; 4] = m_n.transpose().into();
    assert_eq!(t_o, t_n);

    assert_eq!(m_o.determinant(), m_n.determinant());

    let inv_o: Option<[[f64; 4]; 4]> = m_o.invert().map(Into::into);
    let inv_n: Option<[[f64; 4]; 4]> = m_n.invert().map(Into::into);
    assert_eq!(inv_o, inv_n);
}

#[test]
fn quaternion_multiplication_and_vector_rotation() {
    let q1_o = cgmath::Quaternion::from_sv(0.7071067811865476f64, v3_old(0.0, 0.0, 0.7071067811865476));
    let q1_n = cgmath_next::Quaternion::from_sv(0.7071067811865476f64, v3_next(0.0, 0.0, 0.7071067811865476));
    let q2_o = cgmath::Quaternion::from_sv(1.0f64, v3_old(0.1, 0.2, 0.3));
    let q2_n = cgmath_next::Quaternion::from_sv(1.0f64, v3_next(0.1, 0.2, 0.3));

    let prod_o: (f64, f64, f64, f64) = (q1_o * q2_o).into();
    let prod_n: (f64, f64, f64, f64) = (q1_n * q2_n).into();
    assert_eq!(prod_o, prod_n);

    let v_o = v3_old(1.0, 0.0, 0.0);
    let v_n = v3_next(1.0, 0.0, 0.0);
    assert_v3_eq(q1_o.rotate_vector(v_o), q1_n.rotate_vector(v_n));
}

#[test]
fn euler_and_angle_conversion() {
    let deg_o = cgmath::Deg(90.0f64);
    let deg_n = cgmath_next::Deg(90.0f64);
    let rad_o: cgmath::Rad<f64> = deg_o.into();
    let rad_n: cgmath_next::Rad<f64> = deg_n.into();
    assert_eq!(rad_o.0, rad_n.0);

    let euler_o = cgmath::Euler::new(cgmath::Deg(10.0f64), cgmath::Deg(20.0), cgmath::Deg(30.0));
    let euler_n = cgmath_next::Euler::new(
        cgmath_next::Deg(10.0f64),
        cgmath_next::Deg(20.0),
        cgmath_next::Deg(30.0),
    );
    let m_o: [[f64; 3]; 3] = cgmath::Matrix3::from(euler_o).into();
    let m_n: [[f64; 3]; 3] = cgmath_next::Matrix3::from(euler_n).into();
    assert_eq!(m_o, m_n);

    let q_o: (f64, f64, f64, f64) = cgmath::Quaternion::from(euler_o).into();
    let q_n: (f64, f64, f64, f64) = cgmath_next::Quaternion::from(euler_n).into();
    assert_eq!(q_o, q_n);
}

#[test]
fn interpolation_lerp_nlerp_slerp() {
    let a_o = cgmath::Quaternion::from_sv(1.0f64, v3_old(0.0, 0.0, 0.0));
    let a_n = cgmath_next::Quaternion::from_sv(1.0f64, v3_next(0.0, 0.0, 0.0));
    let b_o = cgmath::Quaternion::from_sv(0.0f64, v3_old(1.0, 0.0, 0.0));
    let b_n = cgmath_next::Quaternion::from_sv(0.0f64, v3_next(1.0, 0.0, 0.0));

    let nlerp_o: (f64, f64, f64, f64) = a_o.nlerp(b_o, 0.25).into();
    let nlerp_n: (f64, f64, f64, f64) = a_n.nlerp(b_n, 0.25).into();
    assert_eq!(nlerp_o, nlerp_n);

    let slerp_o: (f64, f64, f64, f64) = a_o.slerp(b_o, 0.25).into();
    let slerp_n: (f64, f64, f64, f64) = a_n.slerp(b_n, 0.25).into();
    assert_eq!(slerp_o, slerp_n);

    let p1_o = cgmath::Point3::new(0.0f64, 0.0, 0.0);
    let p1_n = cgmath_next::Point3::new(0.0f64, 0.0, 0.0);
    let p2_o = cgmath::Point3::new(10.0f64, 20.0, 30.0);
    let p2_n = cgmath_next::Point3::new(10.0f64, 20.0, 30.0);
    let mid_o = p1_o.midpoint(p2_o);
    let mid_n = p1_n.midpoint(p2_n);
    assert_eq!(mid_o.x, mid_n.x);
    assert_eq!(mid_o.y, mid_n.y);
    assert_eq!(mid_o.z, mid_n.z);
}

#[test]
fn look_at_and_projection() {
    let eye_o = cgmath::Point3::new(3.0f64, 4.0, 5.0);
    let eye_n = cgmath_next::Point3::new(3.0f64, 4.0, 5.0);
    let center_o = cgmath::Point3::new(0.0f64, 0.0, 0.0);
    let center_n = cgmath_next::Point3::new(0.0f64, 0.0, 0.0);
    let up_o = v3_old(0.0, 1.0, 0.0);
    let up_n = v3_next(0.0, 1.0, 0.0);

    let look_o: [[f64; 4]; 4] = cgmath::Matrix4::look_at_rh(eye_o, center_o, up_o).into();
    let look_n: [[f64; 4]; 4] = cgmath_next::Matrix4::look_at_rh(eye_n, center_n, up_n).into();
    assert_eq!(look_o, look_n);

    let persp_o: [[f64; 4]; 4] = cgmath::perspective(cgmath::Deg(60.0f64), 16.0 / 9.0, 0.1, 100.0).into();
    let persp_n: [[f64; 4]; 4] =
        cgmath_next::perspective(cgmath_next::Deg(60.0f64), 16.0 / 9.0, 0.1, 100.0).into();
    assert_eq!(persp_o, persp_n);

    let ortho_o: [[f64; 4]; 4] = cgmath::ortho(-1.0f64, 1.0, -1.0, 1.0, 0.1, 100.0).into();
    let ortho_n: [[f64; 4]; 4] = cgmath_next::ortho(-1.0f64, 1.0, -1.0, 1.0, 0.1, 100.0).into();
    assert_eq!(ortho_o, ortho_n);
}

#[test]
fn transform_composition() {
    let d_o = cgmath::Decomposed {
        scale: 2.0f64,
        rot: cgmath::Quaternion::from_sv(1.0, v3_old(0.0, 0.0, 0.0)),
        disp: v3_old(1.0, 2.0, 3.0),
    };
    let d_n = cgmath_next::Decomposed {
        scale: 2.0f64,
        rot: cgmath_next::Quaternion::from_sv(1.0, v3_next(0.0, 0.0, 0.0)),
        disp: v3_next(1.0, 2.0, 3.0),
    };
    let m_o: cgmath::Matrix4<f64> = d_o.into();
    let m_n: cgmath_next::Matrix4<f64> = d_n.into();
    let arr_o: [[f64; 4]; 4] = m_o.into();
    let arr_n: [[f64; 4]; 4] = m_n.into();
    assert_eq!(arr_o, arr_n);
}

// The original coexistence check, kept as-is.
#[test]
fn both_crates_coexist_and_agree() {
    let old = cgmath::Vector3::new(1.0f32, 2.0, 3.0);
    let next = cgmath_next::Vector3::new(1.0f32, 2.0, 3.0);
    assert_eq!(old.x, next.x);
    assert_eq!(old.y, next.y);
    assert_eq!(old.z, next.z);
}

// serde wire-format differential (docs/release-checklist.md outstanding
// item 6): not just "does cargo test --features serde pass" on this crate
// alone, but byte-for-byte JSON equality against real cgmath 0.18.0's
// derive-generated Serialize/Deserialize output, plus round-trip in both
// directions and cross-deserialization (cgmath-next's JSON parses back
// into 0.18.0's type and vice versa). All `#[derive(Serialize,
// Deserialize)]` types use plain public named fields with no `#[serde(...)]`
// attribute overrides in either crate, so this exercises the same derive
// machinery both sides -- a real difference in field name, order, or type
// would show up as `byte_eq == false` here, not just a type-checker pass.
use serde::de::DeserializeOwned;
use serde::Serialize;

fn assert_serde_matches<A, B>(a: &A, b: &B)
where
    A: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
    B: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let ja = serde_json::to_string(a).unwrap();
    let jb = serde_json::to_string(b).unwrap();
    assert_eq!(ja, jb, "serialized JSON differs between 0.18.0 and cgmath-next");

    let a_roundtrip: A = serde_json::from_str(&ja).unwrap();
    assert_eq!(&a_roundtrip, a, "0.18.0 round-trip changed the value");
    let b_roundtrip: B = serde_json::from_str(&jb).unwrap();
    assert_eq!(&b_roundtrip, b, "cgmath-next round-trip changed the value");

    // cross-deserialization: each crate's JSON must parse into the other's type
    let cross_a: A = serde_json::from_str(&jb).unwrap();
    assert_eq!(&cross_a, a, "cgmath-next's JSON didn't deserialize into 0.18.0's type correctly");
    let cross_b: B = serde_json::from_str(&ja).unwrap();
    assert_eq!(&cross_b, b, "0.18.0's JSON didn't deserialize into cgmath-next's type correctly");
}

#[test]
fn serde_vector_byte_exact_roundtrip() {
    assert_serde_matches(&cgmath::Vector1::new(1.5f32), &cgmath_next::Vector1::new(1.5f32));
    assert_serde_matches(&cgmath::Vector1::new(1.5f64), &cgmath_next::Vector1::new(1.5f64));
    assert_serde_matches(&cgmath::Vector2::new(1.5f32, -2.25), &cgmath_next::Vector2::new(1.5f32, -2.25));
    assert_serde_matches(&cgmath::Vector2::new(1.5f64, -2.25), &cgmath_next::Vector2::new(1.5f64, -2.25));
    assert_serde_matches(
        &cgmath::Vector3::new(1.5f32, -2.25, 3.75),
        &cgmath_next::Vector3::new(1.5f32, -2.25, 3.75),
    );
    assert_serde_matches(
        &cgmath::Vector3::new(1.5f64, -2.25, 3.75),
        &cgmath_next::Vector3::new(1.5f64, -2.25, 3.75),
    );
    assert_serde_matches(
        &cgmath::Vector4::new(1.5f32, -2.25, 3.75, -4.125),
        &cgmath_next::Vector4::new(1.5f32, -2.25, 3.75, -4.125),
    );
    assert_serde_matches(
        &cgmath::Vector4::new(1.5f64, -2.25, 3.75, -4.125),
        &cgmath_next::Vector4::new(1.5f64, -2.25, 3.75, -4.125),
    );
}

#[test]
fn serde_point_byte_exact_roundtrip() {
    assert_serde_matches(&cgmath::Point1::new(9.5f32), &cgmath_next::Point1::new(9.5f32));
    assert_serde_matches(&cgmath::Point1::new(9.5f64), &cgmath_next::Point1::new(9.5f64));
    assert_serde_matches(&cgmath::Point2::new(9.5f32, -1.25), &cgmath_next::Point2::new(9.5f32, -1.25));
    assert_serde_matches(&cgmath::Point2::new(9.5f64, -1.25), &cgmath_next::Point2::new(9.5f64, -1.25));
    assert_serde_matches(
        &cgmath::Point3::new(9.5f32, -1.25, 0.75),
        &cgmath_next::Point3::new(9.5f32, -1.25, 0.75),
    );
    assert_serde_matches(
        &cgmath::Point3::new(9.5f64, -1.25, 0.75),
        &cgmath_next::Point3::new(9.5f64, -1.25, 0.75),
    );
}

#[test]
fn serde_matrix_byte_exact_roundtrip() {
    assert_serde_matches(
        &cgmath::Matrix2::new(1.0f32, 2.0, 3.0, 4.0),
        &cgmath_next::Matrix2::new(1.0f32, 2.0, 3.0, 4.0),
    );
    assert_serde_matches(
        &cgmath::Matrix2::new(1.0f64, 2.0, 3.0, 4.0),
        &cgmath_next::Matrix2::new(1.0f64, 2.0, 3.0, 4.0),
    );
    #[rustfmt::skip]
    assert_serde_matches(
        &cgmath::Matrix3::new(1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0),
        &cgmath_next::Matrix3::new(1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0),
    );
    #[rustfmt::skip]
    let (m_o, m_n) = (
        cgmath::Matrix4::new(
            1.0f32, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ),
        cgmath_next::Matrix4::new(
            1.0f32, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ),
    );
    assert_serde_matches(&m_o, &m_n);
}

#[test]
fn serde_quaternion_byte_exact_roundtrip() {
    assert_serde_matches(
        &cgmath::Quaternion::new(1.0f32, 2.0, 3.0, 4.0),
        &cgmath_next::Quaternion::new(1.0f32, 2.0, 3.0, 4.0),
    );
    assert_serde_matches(
        &cgmath::Quaternion::new(1.0f64, 2.0, 3.0, 4.0),
        &cgmath_next::Quaternion::new(1.0f64, 2.0, 3.0, 4.0),
    );
}

#[test]
fn serde_euler_byte_exact_roundtrip() {
    assert_serde_matches(
        &cgmath::Euler::new(cgmath::Rad(0.1f32), cgmath::Rad(0.2), cgmath::Rad(0.3)),
        &cgmath_next::Euler::new(cgmath_next::Rad(0.1f32), cgmath_next::Rad(0.2), cgmath_next::Rad(0.3)),
    );
    assert_serde_matches(
        &cgmath::Euler::new(cgmath::Deg(10.0f64), cgmath::Deg(20.0), cgmath::Deg(30.0)),
        &cgmath_next::Euler::new(cgmath_next::Deg(10.0f64), cgmath_next::Deg(20.0), cgmath_next::Deg(30.0)),
    );
}

#[test]
fn serde_decomposed_transform_byte_exact_roundtrip() {
    let d_o = cgmath::Decomposed {
        scale: 2.0f64,
        rot: cgmath::Quaternion::new(1.0, 0.0, 0.0, 0.0),
        disp: v3_old(1.0, 2.0, 3.0),
    };
    let d_n = cgmath_next::Decomposed {
        scale: 2.0f64,
        rot: cgmath_next::Quaternion::new(1.0, 0.0, 0.0, 0.0),
        disp: v3_next(1.0, 2.0, 3.0),
    };
    assert_serde_matches(&d_o, &d_n);
}
