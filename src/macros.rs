// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
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

//! Utility macros for code generation

#![macro_use]

#[cfg(feature = "simd")]
macro_rules! default_fn {
    { $($tt:tt)* } => { default fn $( $tt )* };
}

#[cfg(not(feature = "simd"))]
macro_rules! default_fn {
    { $($tt:tt)* } => { fn $( $tt )* };
}

/// Generates a binary operator implementation for the permutations of by-ref and by-val
macro_rules! impl_operator {
    // When it is an unary operator
    (<$S:ident: $Constraint:ident> $Op:ident for $Lhs:ty {
        fn $op:ident($x:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!($op(self) -> $Output {
                let $x = self; $body
            });
        }

        impl<'a, $S: $Constraint> $Op for &'a $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!($op(self) -> $Output {
                let $x = self; $body
            });
        }
    };
    // When the right operand is a scalar
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ident> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!($op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!($op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }
    };
    // When the right operand is a compound type
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }

        impl<'a, $S: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }

        impl<'a, 'b, $S: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }
    };
    // When the left operand is a scalar
    ($Op:ident<$Rhs:ident<$S:ident>> for $Lhs:ty {
        fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr }
    }) => {
        impl $Op<$Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }

        impl<'a> $Op<&'a $Rhs<$S>> for $Lhs {
            type Output = $Output;
            #[inline]
            default_fn!( $op(self, other: &'a $Rhs<$S>) -> $Output {
                let ($lhs, $rhs) = (self, other); $body
            });
        }
    };
}

macro_rules! impl_assignment_operator {
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident(&mut $lhs:ident, $rhs:ident) $body:block
    }) => {
        impl<$S: $Constraint + $Op<$S>> $Op<$Rhs> for $Lhs {
            #[inline]
            default_fn!( $op(&mut $lhs, $rhs: $Rhs) $body );
        }
    };
}

macro_rules! fold_array {
    (&$method:ident, { $x:expr }) => {
        *$x
    };
    (&$method:ident, { $x:expr, $y:expr }) => {
        $x.$method(&$y)
    };
    (&$method:ident, { $x:expr, $y:expr, $z:expr }) => {
        $x.$method(&$y).$method(&$z)
    };
    (&$method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => {
        $x.$method(&$y).$method(&$z).$method(&$w)
    };
    ($method:ident, { $x:expr }) => {
        $x
    };
    ($method:ident, { $x:expr, $y:expr }) => {
        $x.$method($y)
    };
    ($method:ident, { $x:expr, $y:expr, $z:expr }) => {
        $x.$method($y).$method($z)
    };
    ($method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => {
        $x.$method($y).$method($z).$method($w)
    };
}

/// Generate array conversion implementations for a compound array type
macro_rules! impl_fixed_array_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$S> Into<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $ArrayN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                // SAFETY: `$ArrayN<$S>` is `#[repr(C)]` with exactly `$n`
                // fields of type `$S` and no padding (`$S: BaseNum` is
                // always a primitive numeric type whose size equals its
                // alignment), so its layout is byte-identical to `[$S; $n]`.
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                // SAFETY: see `AsRef` above; the same layout identity holds
                // for the mutable reference, and `self` is uniquely
                // borrowed here so there is no aliasing.
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $ArrayN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $ArrayN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $ArrayN<$S> {
                // SAFETY: see `AsRef` above.
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $ArrayN<$S> {
                // SAFETY: see `AsMut` above.
                unsafe { mem::transmute(v) }
            }
        }
    }
}

/// Runtime layout guard for the homogeneous-tuple transmutes below
/// (docs/unsafe-audit.md UNSAFE-002). Plain Rust tuple layout is
/// unspecified by the language reference -- this does NOT make the
/// transmute language-guaranteed sound. What it does: verify, on every
/// call, that `$ArrayN<$S>` and `$Tuple` actually have identical size,
/// alignment, and per-field byte offsets on this platform/monomorphization
/// before the transmute runs, and refuse (panic) instead of transmuting
/// if they don't. A hypothetical future layout divergence becomes a loud,
/// immediate failure instead of silent UB. It's a tripwire, not a proof.
///
/// Field offsets are computed via raw-pointer arithmetic on
/// `MaybeUninit` storage, not `core::mem::offset_of!` -- that macro
/// (including its tuple-index support) requires Rust 1.77, newer than
/// this crate's declared MSRV (1.71, see docs/msrv.md). This computes
/// the same thing without it, so the guard doesn't force an MSRV bump.
///
/// SAFETY: `addr_of!` never reads the pointee and never creates a
/// reference, so taking a field's address through it is sound even
/// though the backing `MaybeUninit` storage is never initialized.
/// Subtracting two pointers derived from the same allocation's base
/// address is well-defined `usize` arithmetic, not pointer-offset
/// arithmetic subject to the same-allocation-only restriction.
///
/// Written as a chain of individually-compared scalar offsets, not as
/// two `[usize; N]` arrays compared with `==`. Both forms compute the
/// same thing, but only this one was empirically confirmed (via release-
/// build disassembly of a representative call site, `Vector4<f32>::as_ref`)
/// to fully constant-fold away when the layout actually matches -- the
/// array-typed version left a real runtime load+compare+branch on every
/// call even though every input to it was already a compile-time
/// constant. See docs/unsafe-audit.md UNSAFE-002 for the comparison.
macro_rules! tuple_layout_matches {
    ($ArrayN:ident <$S:ident> { $($field:ident : $index:tt),+ }, $Tuple:ty) => {{
        use std::mem::{align_of, size_of, MaybeUninit};
        use std::ptr::addr_of;

        size_of::<$ArrayN<$S>>() == size_of::<$Tuple>()
            && align_of::<$ArrayN<$S>>() == align_of::<$Tuple>()
            && {
                let s = MaybeUninit::<$ArrayN<$S>>::uninit();
                let s_base = s.as_ptr() as usize;
                let t = MaybeUninit::<$Tuple>::uninit();
                let t_base = t.as_ptr() as usize;
                unsafe {
                    $(
                        (addr_of!((*s.as_ptr()).$field) as usize - s_base)
                            == (addr_of!((*t.as_ptr()).$index) as usize - t_base)
                    )&&+
                }
            }
    }};
}

/// Generate homogeneous tuple conversion implementations for a compound array type
macro_rules! impl_tuple_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident : $index:tt),+ }, $Tuple:ty) => {
        impl<$S> Into<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $ArrayN { $($field),+ } => ($($field),+,) }
            }
        }

        impl<$S> AsRef<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                // SAFETY: guarded -- see `tuple_layout_matches!` above and
                // docs/unsafe-audit.md UNSAFE-002. Panics instead of
                // transmuting if the checked layout invariant doesn't hold.
                assert!(
                    tuple_layout_matches!($ArrayN<$S> { $($field: $index),+ }, $Tuple),
                    "cgmath-next: internal invariant violated -- {} and {} have \
                     diverged in memory layout on this platform; refusing to \
                     transmute (see docs/unsafe-audit.md UNSAFE-002)",
                    stringify!($ArrayN<$S>), stringify!($Tuple)
                );
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                // SAFETY: see `AsRef` above (docs/unsafe-audit.md UNSAFE-002).
                assert!(
                    tuple_layout_matches!($ArrayN<$S> { $($field: $index),+ }, $Tuple),
                    "cgmath-next: internal invariant violated -- {} and {} have \
                     diverged in memory layout on this platform; refusing to \
                     transmute (see docs/unsafe-audit.md UNSAFE-002)",
                    stringify!($ArrayN<$S>), stringify!($Tuple)
                );
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $ArrayN<$S> {
                match v { ($($field),+,) => $ArrayN { $($field: $field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $ArrayN<$S> {
                // SAFETY: see `AsRef` above (docs/unsafe-audit.md UNSAFE-002).
                assert!(
                    tuple_layout_matches!($ArrayN<$S> { $($field: $index),+ }, $Tuple),
                    "cgmath-next: internal invariant violated -- {} and {} have \
                     diverged in memory layout on this platform; refusing to \
                     transmute (see docs/unsafe-audit.md UNSAFE-002)",
                    stringify!($ArrayN<$S>), stringify!($Tuple)
                );
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $ArrayN<$S> {
                // SAFETY: see `AsMut` above (docs/unsafe-audit.md UNSAFE-002).
                assert!(
                    tuple_layout_matches!($ArrayN<$S> { $($field: $index),+ }, $Tuple),
                    "cgmath-next: internal invariant violated -- {} and {} have \
                     diverged in memory layout on this platform; refusing to \
                     transmute (see docs/unsafe-audit.md UNSAFE-002)",
                    stringify!($ArrayN<$S>), stringify!($Tuple)
                );
                unsafe { mem::transmute(v) }
            }
        }
    }
}

/// Generates index operators for a compound type
macro_rules! impl_index_operators {
    ($VectorN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $VectorN<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; $n] = self.as_ref();
                &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $VectorN<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; $n] = self.as_mut();
                &mut v[i]
            }
        }
    };
}

/// Generate `mint` types conversion implementations
#[cfg(feature = "mint")]
macro_rules! impl_mint_conversions {
    ($ArrayN:ident { $($field:ident),+ }, $Mint:ident) => {
        impl<S: Clone> Into<mint::$Mint<S>> for $ArrayN<S> {
            #[inline]
            fn into(self) -> mint::$Mint<S> {
                mint::$Mint::from([$(self.$field),+])
            }
        }

        impl<S> From<mint::$Mint<S>> for $ArrayN<S> {
            #[inline]
            fn from(v: mint::$Mint<S>) -> Self {
                $ArrayN { $( $field: v.$field, )+ }
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/swizzle_operator_macro.rs"));

#[cfg(test)]
mod tuple_layout_guard_tests {
    use vector::Vector4;

    #[test]
    fn accepts_the_real_correct_mapping() {
        // The mapping actually used by `impl_tuple_conversions!`'s call
        // site for Vector4 (src/vector.rs) -- must pass today, or the
        // guard would break every tuple conversion in the crate.
        assert!(tuple_layout_matches!(
            Vector4<f32> { x: 0, y: 1, z: 2, w: 3 },
            (f32, f32, f32, f32)
        ));
        assert!(tuple_layout_matches!(
            Vector4<f64> { x: 0, y: 1, z: 2, w: 3 },
            (f64, f64, f64, f64)
        ));
    }

    #[test]
    fn rejects_a_scrambled_field_to_index_mapping() {
        // Negative control: this is a test of the DETECTOR, not of any
        // real transmute. `tuple_layout_matches!` only reads byte offsets
        // via `addr_of!` on `MaybeUninit` storage -- it never constructs
        // a live reference and never calls `mem::transmute`, so there is
        // nothing here that can be unsound even though the mapping below
        // is deliberately wrong.
        //
        // Real Vector4<f32>'s field offsets are x=0, y=4, z=8, w=12. This
        // invocation deliberately claims the reversed mapping (as if a
        // hypothetical future layout change had reordered the tuple, or
        // the crate's own macro invocation had a typo in its index
        // list) -- since [0,4,8,12] != [12,8,4,0], the guard must reject
        // it. This is the exact failure mode the guard exists to catch.
        assert!(!tuple_layout_matches!(
            Vector4<f32> { x: 3, y: 2, z: 1, w: 0 },
            (f32, f32, f32, f32)
        ));
    }

    #[test]
    fn rejects_a_size_mismatch() {
        // (f64, f64, f64, f64) is twice the size of Vector4<f32> and
        // doesn't even type-check against `impl_tuple_conversions!`'s
        // real call sites -- this exercises `tuple_layout_matches!`'s
        // size_of guard directly by comparing against a same-arity but
        // differently-sized field type.
        assert!(!tuple_layout_matches!(
            Vector4<f32> { x: 0, y: 1, z: 2, w: 3 },
            (f64, f64, f64, f64)
        ));
    }
}
