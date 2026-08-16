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

// mint conversion inventory (docs/release-checklist.md outstanding item 6):
// every mint conversion impl in the crate, not just a sample -- Vector2/3/4,
// Point2/3, Matrix2/3/4 (column orientation specifically, not just field
// pass-through), Quaternion (scalar/vector order specifically), and Euler
// (axis order). All values below are pairwise-distinct so a component swap,
// a transpose, or a scalar/vector mixup would fail these assertions instead
// of accidentally passing. `mint` here is the single directly-depended-on
// crate (unified by Cargo with both cgmath 0.18.0's and cgmath-next's own
// `mint` dependency, same "0.5" requirement), so `cgmath::Vector2: Into<mint::Vector2<_>>`
// and `cgmath_next::Vector2: Into<mint::Vector2<_>>` genuinely produce the
// same mint type, not two incompatible re-exports.

#[test]
fn mint_vector_component_order_and_roundtrip() {
    let v2_o = cgmath::Vector2::new(1.0f32, 2.0);
    let v2_n = cgmath_next::Vector2::new(1.0f32, 2.0);
    let m2_o: mint::Vector2<f32> = v2_o.into();
    let m2_n: mint::Vector2<f32> = v2_n.into();
    assert_eq!((m2_o.x, m2_o.y), (1.0, 2.0));
    assert_eq!((m2_n.x, m2_n.y), (1.0, 2.0));
    assert_eq!(cgmath::Vector2::from(m2_o), v2_o);
    assert_eq!(cgmath_next::Vector2::from(m2_n), v2_n);

    let v3_o = v3_old(1.0, 2.0, 3.0);
    let v3_n = v3_next(1.0, 2.0, 3.0);
    let m3_o: mint::Vector3<f64> = v3_o.into();
    let m3_n: mint::Vector3<f64> = v3_n.into();
    assert_eq!((m3_o.x, m3_o.y, m3_o.z), (1.0, 2.0, 3.0));
    assert_eq!((m3_n.x, m3_n.y, m3_n.z), (1.0, 2.0, 3.0));
    assert_eq!(cgmath::Vector3::from(m3_o), v3_o);
    assert_eq!(cgmath_next::Vector3::from(m3_n), v3_n);

    let v4_o = cgmath::Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let v4_n = cgmath_next::Vector4::new(1.0f32, 2.0, 3.0, 4.0);
    let m4_o: mint::Vector4<f32> = v4_o.into();
    let m4_n: mint::Vector4<f32> = v4_n.into();
    assert_eq!((m4_o.x, m4_o.y, m4_o.z, m4_o.w), (1.0, 2.0, 3.0, 4.0));
    assert_eq!((m4_n.x, m4_n.y, m4_n.z, m4_n.w), (1.0, 2.0, 3.0, 4.0));
    assert_eq!(cgmath::Vector4::from(m4_o), v4_o);
    assert_eq!(cgmath_next::Vector4::from(m4_n), v4_n);
}

#[test]
fn mint_point_component_order_and_roundtrip() {
    let p2_o = cgmath::Point2::new(5.0f32, 6.0);
    let p2_n = cgmath_next::Point2::new(5.0f32, 6.0);
    let m2_o: mint::Point2<f32> = p2_o.into();
    let m2_n: mint::Point2<f32> = p2_n.into();
    assert_eq!((m2_o.x, m2_o.y), (5.0, 6.0));
    assert_eq!((m2_n.x, m2_n.y), (5.0, 6.0));
    assert_eq!(cgmath::Point2::from(m2_o), p2_o);
    assert_eq!(cgmath_next::Point2::from(m2_n), p2_n);

    let p3_o = cgmath::Point3::new(5.0f64, 6.0, 7.0);
    let p3_n = cgmath_next::Point3::new(5.0f64, 6.0, 7.0);
    let m3_o: mint::Point3<f64> = p3_o.into();
    let m3_n: mint::Point3<f64> = p3_n.into();
    assert_eq!((m3_o.x, m3_o.y, m3_o.z), (5.0, 6.0, 7.0));
    assert_eq!((m3_n.x, m3_n.y, m3_n.z), (5.0, 6.0, 7.0));
    assert_eq!(cgmath::Point3::from(m3_o), p3_o);
    assert_eq!(cgmath_next::Point3::from(m3_n), p3_n);
}

#[test]
fn mint_matrix_column_orientation_and_roundtrip() {
    // All 16 components distinct so a transpose (row/column swap) cannot
    // accidentally produce the same result as the correct column mapping.
    #[rustfmt::skip]
    let m_o = cgmath::Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    #[rustfmt::skip]
    let m_n = cgmath_next::Matrix4::new(
        1.0f64, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let mint_o: mint::ColumnMatrix4<f64> = m_o.into();
    let mint_n: mint::ColumnMatrix4<f64> = m_n.into();

    // mint's .x/.y/.z/.w must equal cgmath's own columns (Matrix::column),
    // not its rows -- this is the actual "column orientation" claim.
    for (i, col) in [mint_o.x, mint_o.y, mint_o.z, mint_o.w].into_iter().enumerate() {
        use cgmath::Matrix as _;
        let expected = m_o[i];
        assert_eq!((col.x, col.y, col.z, col.w), (expected.x, expected.y, expected.z, expected.w));
        // and NOT the row (would only coincide if the matrix were
        // symmetric, which this one deliberately isn't: row 0 is
        // [1,5,9,13], column 0 is [1,2,3,4]).
        let row = m_o.row(i);
        assert_ne!((col.x, col.y, col.z, col.w), (row.x, row.y, row.z, row.w));
    }
    for (i, col) in [mint_n.x, mint_n.y, mint_n.z, mint_n.w].into_iter().enumerate() {
        use cgmath_next::Matrix as _;
        let expected = m_n[i];
        assert_eq!((col.x, col.y, col.z, col.w), (expected.x, expected.y, expected.z, expected.w));
        let row = m_n.row(i);
        assert_ne!((col.x, col.y, col.z, col.w), (row.x, row.y, row.z, row.w));
    }

    assert_eq!(cgmath::Matrix4::from(mint_o), m_o);
    assert_eq!(cgmath_next::Matrix4::from(mint_n), m_n);

    // Matrix2/Matrix3 round-trip (orientation already proven above; these
    // just confirm the inventory covers every matrix size, not just 4x4).
    let m2_o = cgmath::Matrix2::new(1.0f32, 2.0, 3.0, 4.0);
    let m2_n = cgmath_next::Matrix2::new(1.0f32, 2.0, 3.0, 4.0);
    let mint2_o: mint::ColumnMatrix2<f32> = m2_o.into();
    let mint2_n: mint::ColumnMatrix2<f32> = m2_n.into();
    assert_eq!((mint2_o.x.x, mint2_o.x.y, mint2_o.y.x, mint2_o.y.y), (1.0, 2.0, 3.0, 4.0));
    assert_eq!(cgmath::Matrix2::from(mint2_o), m2_o);
    assert_eq!(cgmath_next::Matrix2::from(mint2_n), m2_n);

    #[rustfmt::skip]
    let m3_o = cgmath::Matrix3::new(1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    #[rustfmt::skip]
    let m3_n = cgmath_next::Matrix3::new(1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let mint3_o: mint::ColumnMatrix3<f32> = m3_o.into();
    let mint3_n: mint::ColumnMatrix3<f32> = m3_n.into();
    assert_eq!((mint3_o.x.x, mint3_o.x.y, mint3_o.x.z), (1.0, 2.0, 3.0));
    assert_eq!(cgmath::Matrix3::from(mint3_o), m3_o);
    assert_eq!(cgmath_next::Matrix3::from(mint3_n), m3_n);
}

#[test]
fn mint_quaternion_scalar_vector_order_and_roundtrip() {
    // Quaternion::new(w, xi, yj, zk) -- w is the scalar part, (xi,yj,zk)
    // the vector part. All 4 components distinct.
    let q_o = cgmath::Quaternion::new(4.0f64, 1.0, 2.0, 3.0);
    let q_n = cgmath_next::Quaternion::new(4.0f64, 1.0, 2.0, 3.0);
    let m_o: mint::Quaternion<f64> = q_o.into();
    let m_n: mint::Quaternion<f64> = q_n.into();

    // mint::Quaternion.s must be the scalar (4.0, i.e. w), and .v the
    // vector part (1,2,3) -- not shuffled or reversed.
    assert_eq!(m_o.s, 4.0);
    assert_eq!((m_o.v.x, m_o.v.y, m_o.v.z), (1.0, 2.0, 3.0));
    assert_eq!(m_n.s, 4.0);
    assert_eq!((m_n.v.x, m_n.v.y, m_n.v.z), (1.0, 2.0, 3.0));

    assert_eq!(cgmath::Quaternion::from(m_o), q_o);
    assert_eq!(cgmath_next::Quaternion::from(m_n), q_n);
}

#[test]
fn mint_euler_axis_order_and_roundtrip() {
    // The `From`/`Into` bounds on `Euler<A>`'s mint conversion are
    // `A: From<S>`/`A: Into<S>` for mint's generic scalar `S` -- since
    // `Rad<f64>` doesn't implement `From<f64>`/`Into<f64>` (only
    // `From<Deg<f64>>`, see src/angle.rs), the only S that actually
    // satisfies the bound is the Angle type itself (S = Rad<f64>, via
    // the reflexive `impl<T> From<T> for T`), not the bare scalar.
    let e_o = cgmath::Euler::new(cgmath::Rad(0.1f64), cgmath::Rad(0.2), cgmath::Rad(0.3));
    let e_n = cgmath_next::Euler::new(cgmath_next::Rad(0.1f64), cgmath_next::Rad(0.2), cgmath_next::Rad(0.3));
    let m_o: mint::EulerAngles<cgmath::Rad<f64>, mint::IntraXYZ> = e_o.into();
    let m_n: mint::EulerAngles<cgmath_next::Rad<f64>, mint::IntraXYZ> = e_n.into();

    // .a/.b/.c must map to x/y/z in that order, not shuffled.
    assert_eq!((m_o.a.0, m_o.b.0, m_o.c.0), (0.1, 0.2, 0.3));
    assert_eq!((m_n.a.0, m_n.b.0, m_n.c.0), (0.1, 0.2, 0.3));

    let e_o2: cgmath::Euler<cgmath::Rad<f64>> = m_o.into();
    let e_n2: cgmath_next::Euler<cgmath_next::Rad<f64>> = m_n.into();
    assert_eq!((e_o2.x.0, e_o2.y.0, e_o2.z.0), (0.1, 0.2, 0.3));
    assert_eq!((e_n2.x.0, e_n2.y.0, e_n2.z.0), (0.1, 0.2, 0.3));
}
