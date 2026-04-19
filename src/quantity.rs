use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{
    Dimension, Scalar, UnitOf,
    dimension::{DimDiv, DimMul},
};

#[derive(Copy, Debug)]
pub struct Quantity<D: Dimension, S: Scalar = f32> {
    value: S,
    _marker: std::marker::PhantomData<D>,
}

impl<D: Dimension, S: Scalar> Quantity<D, S> {
    const fn new(value: S) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}

const impl<D: Dimension, S: [const] Scalar> Quantity<D, S> {
    #[inline]
    pub fn to<U: [const] UnitOf<D>>(&self) -> S {
        U::convert_from_canonical(self.value.clone())
    }

    /// Creates a new dimension from the given scalar and unit.
    ///
    /// # Note
    ///
    /// Usage of this function directly is discouraged. Instead, use multiplication or the `ScalarExt` trait.
    ///
    /// ```
    /// # use shrewnit::prelude::*;
    ///
    /// let quantity = 30.0f32 * Meters;
    /// let quantity = 30.0f32.meters();
    /// ```
    #[inline]
    pub fn from_scalar<U: [const] UnitOf<D>>(value: S) -> Self {
        Self::from_canonical(U::convert_to_canonical(value))
    }

    /// Returns the canonical representation of the dimension.
    #[inline]
    pub fn canonical(&self) -> S {
        self.value.clone()
    }
    /// Creates a new dimension from the canonical representation.
    #[inline]
    pub fn from_canonical(value: S) -> Self {
        Self::new(value)
    }
}

// Traits that can't be const_derived because of PhantomData

impl<D: [const] Dimension, S: [const] Scalar> const Clone for Quantity<D, S> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<D: [const] Dimension, S: [const] Scalar + [const] PartialEq> const PartialEq
    for Quantity<D, S>
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<D: [const] Dimension, S: [const] Scalar + [const] Eq> const Eq for Quantity<D, S> {}

impl<D: [const] Dimension, S: [const] Scalar + [const] Ord> const Ord for Quantity<D, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
impl<D: [const] Dimension, S: [const] Scalar + [const] PartialOrd> const PartialOrd
    for Quantity<D, S>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

// MDAS overloads

impl<D: [const] Dimension, S: [const] Scalar> const Mul<S> for Quantity<D, S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        Self::new(self.value * rhs)
    }
}
impl<D: [const] Dimension, S: [const] Scalar> const MulAssign<S> for Quantity<D, S> {
    fn mul_assign(&mut self, rhs: S) {
        self.value = self.value.clone() * rhs
    }
}

impl<D: [const] Dimension, S: [const] Scalar> const Div<S> for Quantity<D, S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        Self::new(self.value / rhs)
    }
}
impl<D: [const] Dimension, S: [const] Scalar> const DivAssign<S> for Quantity<D, S> {
    fn div_assign(&mut self, rhs: S) {
        self.value = self.value.clone() / rhs
    }
}

impl<D: [const] Dimension, S: [const] Scalar> const Add for Quantity<D, S> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}
impl<D: [const] Dimension, S: [const] Scalar> const AddAssign for Quantity<D, S> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = self.value.clone() + rhs.value
    }
}

impl<D: [const] Dimension, S: [const] Scalar> const Sub for Quantity<D, S> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}
impl<D: [const] Dimension, S: [const] Scalar> const SubAssign for Quantity<D, S> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = self.value.clone() - rhs.value
    }
}

// Other operations (%, -)

// Operations with other quantities

impl<
    LhsDim: [const] Dimension + [const] DimMul<RhsDim>,
    RhsDim: [const] Dimension,
    S: [const] Scalar,
> const Mul<Quantity<RhsDim, S>> for Quantity<LhsDim, S>
{
    type Output = Quantity<LhsDim::Output, S>;

    fn mul(self, rhs: Quantity<RhsDim, S>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

impl<LhsDim: [const] Dimension + DimDiv<RhsDim>, RhsDim: [const] Dimension, S: [const] Scalar> const
    Div<Quantity<RhsDim, S>> for Quantity<LhsDim, S>
{
    type Output = Quantity<LhsDim::Output, S>;

    fn div(self, rhs: Quantity<RhsDim, S>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}
