// Copyright 2013-2017 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Dedicated verification for the `rand`-gated `Distribution` impls
//! (docs/release-checklist.md outstanding item 6). Confirms the existing
//! contract these impls have always had -- generated component values are
//! finite and (where the source code determines a specific bound) within
//! that bound -- rather than assuming "it compiles and runs" is sufficient.
//! Does NOT assert exact RNG output sequences: upstream 0.18.0 never
//! documented or guaranteed a specific output for a given seed, only that
//! `Standard: Distribution<T>` is implemented and produces values of the
//! right type, so pinning an exact sequence here would test an
//! implementation detail this crate (and upstream) never promised.

#![cfg(feature = "rand")]

extern crate cgmath;
extern crate rand;

use cgmath::{
    Deg, Euler, InnerSpace, Matrix2, Matrix3, Matrix4, Quaternion, Rad, Vector1, Vector2, Vector3,
    Vector4,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const SAMPLES: usize = 256;
const SEED: u64 = 0x5EED_5EED_5EED_5EED;

fn rng() -> SmallRng {
    SmallRng::seed_from_u64(SEED)
}

#[test]
fn vector_components_are_finite_and_in_standard_range() {
    let mut rng = rng();
    for _ in 0..SAMPLES {
        let v: Vector1<f64> = rng.gen();
        assert!(v.x.is_finite() && (0.0..1.0).contains(&v.x));
    }
    for _ in 0..SAMPLES {
        let v: Vector2<f32> = rng.gen();
        for c in [v.x, v.y] {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
    for _ in 0..SAMPLES {
        let v: Vector3<f64> = rng.gen();
        for c in [v.x, v.y, v.z] {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
    for _ in 0..SAMPLES {
        let v: Vector4<f32> = rng.gen();
        for c in [v.x, v.y, v.z, v.w] {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
}

#[test]
fn matrix_components_are_finite_and_in_standard_range() {
    let mut rng = rng();
    for _ in 0..SAMPLES {
        let m: Matrix2<f64> = rng.gen();
        let cols: [[f64; 2]; 2] = m.into();
        for c in cols.iter().flatten().copied() {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
    for _ in 0..SAMPLES {
        let m: Matrix3<f64> = rng.gen();
        let cols: [[f64; 3]; 3] = m.into();
        for c in cols.iter().flatten().copied() {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
    for _ in 0..SAMPLES {
        let m: Matrix4<f32> = rng.gen();
        let cols: [[f32; 4]; 4] = m.into();
        for c in cols.iter().flatten().copied() {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
}

#[test]
fn quaternion_components_are_finite_and_in_standard_range() {
    let mut rng = rng();
    for _ in 0..SAMPLES {
        let q: Quaternion<f64> = rng.gen();
        let (xi, yj, zk, w): (f64, f64, f64, f64) = q.into();
        for c in [xi, yj, zk, w] {
            assert!(c.is_finite() && (0.0..1.0).contains(&c));
        }
    }
}

#[test]
fn angle_components_are_finite_and_within_documented_bound() {
    // impl_angle!(Rad, ..., hi = PI): sample range is [-PI, PI).
    let mut rng = rng();
    for _ in 0..SAMPLES {
        let r: Rad<f64> = rng.gen();
        assert!(r.0.is_finite());
        assert!(
            (-std::f64::consts::PI..std::f64::consts::PI).contains(&r.0),
            "Rad sample {} outside documented [-pi, pi) range",
            r.0
        );
    }
    // impl_angle!(Deg, ..., hi = 180): sample range is [-180, 180).
    for _ in 0..SAMPLES {
        let d: Deg<f64> = rng.gen();
        assert!(d.0.is_finite());
        assert!(
            (-180.0..180.0).contains(&d.0),
            "Deg sample {} outside documented [-180, 180) range",
            d.0
        );
    }
}

#[test]
fn euler_components_are_finite_and_within_bound() {
    let mut rng = rng();
    for _ in 0..SAMPLES {
        let e: Euler<Rad<f64>> = rng.gen();
        for a in [e.x, e.y, e.z] {
            assert!(a.0.is_finite());
            assert!((-std::f64::consts::PI..std::f64::consts::PI).contains(&a.0));
        }
    }
}

// Not a strict statistical test (that would be flaky by nature) -- just a
// sanity check that samples aren't degenerate (e.g. a broken RNG wiring
// that always returns the same value, or a component that's always zero).
#[test]
fn samples_are_not_degenerate() {
    let mut rng = rng();
    let samples: Vec<Vector3<f64>> = (0..SAMPLES).map(|_| rng.gen()).collect();
    let distinct = samples.windows(2).filter(|w| w[0] != w[1]).count();
    assert!(
        distinct > SAMPLES / 2,
        "samples look degenerate: too many consecutive duplicates"
    );
    // Also sanity-check the vectors aren't all unit-length or all zero,
    // which would suggest something other than independent per-component
    // sampling.
    let all_zero = samples.iter().all(|v| v.magnitude2() == 0.0);
    assert!(!all_zero);
}
