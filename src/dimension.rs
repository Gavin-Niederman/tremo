use crate::{dimension, quantity::Quantity};

pub trait Dimension {}

/// A trait to find the resulting dimension for quantity multiplications
pub trait DimMul<Rhs: Dimension> {
    type Output: Dimension;
}
/// A trait to find the resulting dimension for quantity multiplications
pub trait DimDiv<Rhs: Dimension> {
    type Output: Dimension;
}

dimension! {
    pub dim Dimensionless {
        Value: 1.0 per canonical,
    } where {
        Self / any,
    }
}
impl<R: Dimension> DimMul<R> for Dimensionless {
    type Output = R;
}

/// Marker trait for dimensions made with operations. (e.g Per, Mul)
pub trait OperationDimension {}

pub struct Per<Num: Dimension, Den: Dimension>(std::marker::PhantomData<(Num, Den)>);
pub struct Mul<L: Dimension, R: Dimension>(std::marker::PhantomData<(L, R)>);

impl<L: Dimension, R: Dimension> OperationDimension for Per<L, R> {}
impl<L: Dimension, R: Dimension> OperationDimension for Mul<L, R> {}

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
pub enum DOverDMarkerRight {}
pub enum DOverDMarkerLeft {}

impl<Shared: Dimension> Simplify<DOverDMarkerRight> for Per<Shared, Shared> {
    type Simplified = Dimensionless;
}
impl<Num: Dimension, Shared: Dimension> Simplify<DOverDMarkerRight>
    for Mul<Per<Num, Shared>, Shared>
{
    type Simplified = Num;
}

impl<Num: Dimension, Shared: Dimension> Simplify<DOverDMarkerRight>
    for Per<Mul<Num, Shared>, Shared>
{
    type Simplified = Num;
}
impl<Num: Dimension, Shared: Dimension> Simplify<DOverDMarkerLeft>
    for Per<Mul<Shared, Num>, Shared>
{
    type Simplified = Num;
}

// Dimensionless cases
pub enum DimensionlessOpMarkerLeft {}
pub enum DimensionlessOpMarkerRight {}

impl<Num: Dimension> Simplify<DimensionlessOpMarkerLeft> for Per<Num, Dimensionless> {
    type Simplified = Num;
}
impl<L: Dimension> Simplify<DimensionlessOpMarkerLeft> for Mul<L, Dimensionless> {
    type Simplified = L;
}
impl<R: Dimension> Simplify<DimensionlessOpMarkerRight> for Mul<Dimensionless, R> {
    type Simplified = R;
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
