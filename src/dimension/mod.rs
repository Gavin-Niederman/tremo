pub mod simplify;

use crate::__dimension_raw;

pub trait Dimension {}

/// A trait to find the resulting dimension for quantity multiplications
pub trait DimMul<Rhs: Dimension> {
    type Output: Dimension;
}
/// A trait to find the resulting dimension for quantity multiplications
pub trait DimDiv<Rhs: Dimension> {
    type Output: Dimension;
}

__dimension_raw! {
    pub dim Dimensionless {
        Value: 1.0 per canonical,
    }
}
impl<R: Dimension> DimMul<R> for Dimensionless {
    type Output = R;
}
impl<R: Dimension> DimDiv<R> for Dimensionless {
    type Output = Per<Dimensionless, R>;
}

#[derive(Clone, Copy)]
pub struct Per<Num: Dimension, Den: Dimension>(std::marker::PhantomData<(Num, Den)>);

#[derive(Clone, Copy)]
pub struct Mul<L: Dimension, R: Dimension>(std::marker::PhantomData<(L, R)>);

pub trait Commute: Dimension {
    type Commuted: Dimension;
}

impl<L: Dimension, R: Dimension> Commute for Mul<L, R> {
    type Commuted = Mul<R, L>;
}

impl<Num: Dimension, Den: Dimension> Dimension for Per<Num, Den> {}
impl<Num: Dimension, Den: Dimension, Rhs: Dimension> DimDiv<Rhs> for Per<Num, Den> {
    type Output = Per<Self, Rhs>;
}
impl<Num: Dimension, Den: Dimension, Rhs: Dimension> DimMul<Rhs> for Per<Num, Den> {
    type Output = Mul<Self, Rhs>;
}

impl<L: Dimension, R: Dimension> Dimension for Mul<L, R> {}
impl<L: Dimension, R: Dimension, Rhs: Dimension> DimDiv<Rhs> for Mul<L, R> {
    type Output = Per<Self, Rhs>;
}
impl<L: Dimension, R: Dimension, Rhs: Dimension> DimMul<Rhs> for Mul<L, R> {
    type Output = Mul<Self, Rhs>;
}
