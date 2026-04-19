use crate::{UnitOf, quantity::Quantity};

pub const trait Dimension {}

/// A trait to find the resulting dimension for quantity multiplications
pub const trait DimMul<Rhs: Dimension> {
    type Output: Dimension;
}
/// A trait to find the resulting dimension for quantity multiplications
pub const trait DimDiv<Rhs: Dimension> {
    type Output: Dimension;
}

pub enum Dimensionless {}
impl const Dimension for Dimensionless {}

/// Marker trait for dimensions made with operations. (e.g Per, Mul)
pub trait OperationDimension {}

pub struct Per<Num: Dimension, Den: Dimension>(std::marker::PhantomData<(Num, Den)>);
pub struct Mul<L: Dimension, R: Dimension>(std::marker::PhantomData<(L, R)>);

impl<L: Dimension, R: Dimension> OperationDimension for Per<L, R> {}
impl<L: Dimension, R: Dimension> OperationDimension for Mul<L, R> {}

impl<Num: Dimension, Den: Dimension> const Dimension for Per<Num, Den> {}
impl<Num: Dimension, Den: Dimension, Rhs: Dimension> const DimDiv<Rhs> for Per<Num, Den> {
    type Output = Per<Self, Rhs>;
}
impl<Num: Dimension, Den: Dimension, Rhs: Dimension> const DimMul<Rhs> for Per<Num, Den> {
    type Output = Mul<Self, Rhs>;
}

impl<L: Dimension, R: Dimension> const Dimension for Mul<L, R> {}
impl<L: Dimension, R: Dimension, Rhs: Dimension> const DimDiv<Rhs> for Mul<L, R> {
    type Output = Per<Self, Rhs>;
}
impl<L: Dimension, R: Dimension, Rhs: Dimension> const DimMul<Rhs> for Mul<L, R> {
    type Output = Mul<Self, Rhs>;
}

pub const trait Simplify<Marker> {
    type Simplified: Dimension;
}

// shared numerator and denominator cases
pub enum DOverDMarkerRight {}
pub enum DOverDMarkerLeft {}

impl<Shared: Dimension> const Simplify<DOverDMarkerRight> for Per<Shared, Shared> {
    type Simplified = Dimensionless;
}
impl<Num: Dimension, Shared: Dimension> const Simplify<DOverDMarkerRight>
    for Mul<Per<Num, Shared>, Shared>
{
    type Simplified = Num;
}

impl<Num: Dimension, Shared: Dimension> const Simplify<DOverDMarkerRight>
    for Per<Mul<Num, Shared>, Shared>
{
    type Simplified = Num;
}
impl<Num: Dimension, Shared: Dimension> const Simplify<DOverDMarkerLeft>
    for Per<Mul<Shared, Num>, Shared>
{
    type Simplified = Num;
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
