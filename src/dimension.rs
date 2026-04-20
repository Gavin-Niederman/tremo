use crate::{__dimension_raw, quantity::Quantity};

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

pub struct Per<Num: Dimension, Den: Dimension>(std::marker::PhantomData<(Num, Den)>);
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

pub trait Simplify<Marker> {
    type Simplified: Dimension;
}

// shared numerator and denominator cases
pub enum DOverDMarker {}

impl<Shared: Dimension> Simplify<DOverDMarker> for Per<Shared, Shared> {
    type Simplified = Dimensionless;
}
impl<Num: Dimension, Shared: Dimension> Simplify<DOverDMarker> for Mul<Per<Num, Shared>, Shared> {
    type Simplified = Num;
}

impl<Num: Dimension, Shared: Dimension> Simplify<DOverDMarker> for Per<Mul<Num, Shared>, Shared> {
    type Simplified = Num;
}

// Commutable Cases
pub enum CommutedMarker {}

impl<L: Dimension, R: Dimension, M> Simplify<(CommutedMarker, M)> for Mul<L, R>
where
    <Mul<L, R> as Commute>::Commuted: Simplify<M>,
{
    type Simplified = <<Mul<L, R> as Commute>::Commuted as Simplify<M>>::Simplified;
}

impl<Num: Dimension + Commute> Simplify<CommutedMarker> for Per<Num, Num::Commuted> {
    type Simplified = Dimensionless;
}

// Dimensionless cases
pub enum DimensionlessOpMarker {}

impl<Num: Dimension> Simplify<DimensionlessOpMarker> for Per<Num, Dimensionless> {
    type Simplified = Num;
}
impl<L: Dimension> Simplify<DimensionlessOpMarker> for Mul<L, Dimensionless> {
    type Simplified = L;
}

// Inner cases
pub enum InnerSimplifiable {}

// Division is not commutable
pub enum InnerSimplifiableDen {}
pub enum InnerSimplifiableNum {}

impl<M, L: Dimension + Simplify<M>, R: Dimension> Simplify<(InnerSimplifiable, M)> for Mul<L, R> {
    type Simplified = Mul<L::Simplified, R>;
}

impl<M, Num: Dimension + Simplify<M>, Den: Dimension> Simplify<(InnerSimplifiableNum, M)>
    for Per<Num, Den>
{
    type Simplified = Per<Num::Simplified, Den>;
}
impl<M, Num: Dimension, Den: Dimension + Simplify<M>> Simplify<(InnerSimplifiableDen, M)>
    for Per<Num, Den>
{
    type Simplified = Per<Num, Den::Simplified>;
}

// Base case
// pub enum BaseMarker {}

// impl<D: Dimension> Simplify<BaseMarker> for D {
//     type Simplified = D;
// }

pub const fn simplify<M, D: Dimension + Simplify<M>>(
    quantity: Quantity<D>,
) -> Quantity<D::Simplified> {
    let value = quantity.canonical();
    Quantity::from_canonical(value)
}
