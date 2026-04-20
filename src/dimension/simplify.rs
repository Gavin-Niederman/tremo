use crate::{
    dimension::{Commute as Commutable, Dimension, Dimensionless, Mul, Per},
    quantity::Quantity,
};

pub trait Simplify<Marker> {
    type Simplified: Dimension;
}

// shared numerator and denominator cases

/// 'Cancels' out a shared numerator and denominator. Works in any of the following cases:
///
/// - S / S -> Dimensionless
/// - (N / S) * S -> N
/// - (N * S) / S -> N
pub enum Cancel {}

impl<Shared: Dimension> Simplify<Cancel> for Per<Shared, Shared> {
    type Simplified = Dimensionless;
}
impl<Num: Dimension, Shared: Dimension> Simplify<Cancel> for Mul<Per<Num, Shared>, Shared> {
    type Simplified = Num;
}

impl<Num: Dimension, Shared: Dimension> Simplify<Cancel> for Per<Mul<Num, Shared>, Shared> {
    type Simplified = Num;
}

// Dimensionless cases

/// Simplifies multiplications and divisions with Dimensionless (N / Dimensionless -> N, N * Dimensionless -> N)
pub enum NoDim {}

impl<Num: Dimension> Simplify<NoDim> for Per<Num, Dimensionless> {
    type Simplified = Num;
}
impl<L: Dimension> Simplify<NoDim> for Mul<L, Dimensionless> {
    type Simplified = L;
}

// Inner cases
//
// The terminology is confusing here.
// from the trait implementation perspective, we are simplifying an operand of the dimension.
// from the user's perspective we are doing nothing at this *level* of the dimension type and passing to the generic of Pass.
// For this reason we call the marker pass even though for the implementation it would make more sense to call it Inner or something similar.

/// 'Passes' this level of the dimension type and performs an operation on whichever side of the operation 'M' applies to.
///
/// This exclusively works with commutative operations ([`Mul`]).
/// See [`PassL`] and [`PassR`] for use with [`Per`]
pub struct Pass<M>(M);

// Division is not commutable

/// 'Passes' this level of the dimension type and performs the operation `M` on the numerator/left operand
///
/// This exclusively works with non-commutative operations ([`Per`]).
/// See [`Pass`] for use with [`Mul`]
pub struct PassL<M>(M);
/// 'Passes' this level of the dimension type and performs the operation `M` on the denominator/right operand
///
/// This exclusively works with non-commutative operations ([`Per`]).
/// See [`Pass`] for use with [`Mul`]
pub struct PassR<M>(M);

impl<M, L: Dimension + Simplify<M>, R: Dimension> Simplify<Pass<M>> for Mul<L, R> {
    type Simplified = Mul<L::Simplified, R>;
}

impl<M, Num: Dimension + Simplify<M>, Den: Dimension> Simplify<PassL<M>> for Per<Num, Den> {
    type Simplified = Per<Num::Simplified, Den>;
}
impl<M, Num: Dimension, Den: Dimension + Simplify<M>> Simplify<PassR<M>> for Per<Num, Den> {
    type Simplified = Per<Num, Den::Simplified>;
}

// Commutable Cases

/// Commutes either a multiplication operation: L * R -> R * L, or the numerator in a division: (L * R) / (R * L) -> (R * L) / (R * L).
/// Useful for acheiving a [`Cancel`] simplification
pub struct Commute<M>(M);

// Prevent infinite depth commution by implementing for each marker type explicitly excluding Commute<M>
macro_rules! commute_impl_mul {
    (@simple $($marker:ty),*) => {
        $(
            impl<L: Dimension, R: Dimension> Simplify<Commute<$marker>> for Mul<L, R>
            where
                <Mul<L, R> as Commutable>::Commuted: Simplify<$marker>,
            {
                type Simplified = <<Mul<L, R> as Commutable>::Commuted as Simplify<$marker>>::Simplified;
            }
        )*
    };
    (@wrapper $($marker:ty),*) => {
        $(
            impl<L: Dimension, R: Dimension, M> Simplify<Commute<$marker>> for Mul<L, R>
            where
                <Mul<L, R> as Commutable>::Commuted: Simplify<$marker>,
            {
                type Simplified = <<Mul<L, R> as Commutable>::Commuted as Simplify<$marker>>::Simplified;
            }
        )*
    };
}
commute_impl_mul!(@simple Cancel, NoDim);
commute_impl_mul!(@wrapper Pass<M>, PassR<M>, PassL<M>);

impl<Num: Dimension + Commutable, Den: Dimension, M> Simplify<Commute<M>> for Per<Num, Den>
where
    Per<Num::Commuted, Den>: Simplify<M>,
{
    type Simplified = <Per<Num::Commuted, Den> as Simplify<M>>::Simplified;
}

// Base case
// pub enum Nop {}

// impl<D: Dimension> Simplify<Nop> for D {
//     type Simplified = D;
// }

pub const fn simplify<M, D: Dimension + Simplify<M>>(
    quantity: Quantity<D>,
) -> Quantity<D::Simplified> {
    let value = quantity.canonical();
    Quantity::from_canonical(value)
}
