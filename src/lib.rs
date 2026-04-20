#![feature(
    const_trait_impl,
    const_destruct,
    const_ops,
    const_cmp,
    const_clone,
    // specialization
)]

use core::marker::Sized;
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Sub},
};

use crate::dimension::Dimension;

pub mod dimension;
pub mod macros;
pub mod premade;
pub mod quantity;

pub const trait Scalar:
    [const] Destruct
    + [const] Clone
    + [const] Mul<Output = Self>
    + [const] Div<Output = Self>
    + [const] Sub<Output = Self>
    + [const] Add<Output = Self>
{
    fn from_f64(value: f64) -> Self;
}

macro_rules! primitive_scalar_imp {
    ($($ty:ty),*) => {
        $(
            impl const $crate::Scalar for $ty {
                fn from_f64(value: f64) -> Self {
                    value as $ty
                }
            }
        )*
    };
}
primitive_scalar_imp!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, usize, isize, i128, f32, f64
);

pub const trait UnitOf<D: Dimension + ?Sized> {
    fn convert_to_canonical<S: [const] Scalar>(canonical: S) -> S;
    fn convert_from_canonical<S: [const] Scalar>(canonical: S) -> S;
}

pub trait CanonicalUnit: Dimension {
    type Canonical: UnitOf<Self>;
}
pub trait Abbreviate {
    const ABBREVIATION: &str;
}
