//! Macros for defining dimensions and units.

/// Implements `Scalar * Unit` for each listed primitive type.
#[macro_export]
#[doc(hidden)]
macro_rules! __unit_mult_imp {
    ($unit:ident, $dimension:ident, $($prim:ty),+) => {
        $(
            impl const core::ops::Mul<$unit> for $prim {
                type Output = $crate::quantity::Quantity<$dimension, $prim>;
                fn mul(self, _rhs: $unit) -> Self::Output {
                    $crate::quantity::Quantity::<_, $prim>::from_scalar::<$unit>(self)
                }
            }
        )+
    };
}

/// Recursively emits `DimMul` / `DimDiv` impls from the `where { … }` block of `dimension!`.
#[macro_export]
#[doc(hidden)]
macro_rules! __measure_conversions {
    {} => {};
    {$self:ty,} => {};
    ($self:ident, Self * $rhs:ident => $output:ident, $($rest:tt)*) => {
        impl $crate::dimension::DimMul<$rhs> for $self {
            type Output = $output;
        }
        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self / $rhs:ident => $output:ident, $($rest:tt)*) => {
        impl $crate::dimension::DimDiv<$rhs> for $self {
            type Output = $output;
        }
        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self * any, $($rest:tt)*) => {
        impl<D: $crate::dimension::Dimension> $crate::dimension::DimMul<D> for $self {
            type Output = $crate::dimension::Mul<Self, D>;
        }
        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self / any, $($rest:tt)*) => {
        impl<D: $crate::dimension::Dimension> $crate::dimension::DimDiv<D> for $self {
            type Output = $crate::dimension::Per<Self, D>;
        }
        $crate::__measure_conversions!($self, $($rest)*);
    }
}

/// Creates a bare unit struct and implements `Unit * Scalar` and `Scalar * Unit`.
///
/// You must still implement [`UnitOf`](crate::UnitOf) for the new type.
///
/// # Examples
///
/// ```
/// use tuant::{premade::Length, UnitOf};
///
/// tuant::unit_type!(
///     /// A unit of length.
///     pub Feet of dimension Length
/// );
///
/// impl const tuant::UnitOf<Length> for Feet {
///     fn convert_to_canonical<S: [const] tuant::Scalar>(canonical: S) -> S {
///         canonical * S::from_f64(3.28084)
///     }
///     fn convert_from_canonical<S: [const] tuant::Scalar>(canonical: S) -> S {
///         canonical / S::from_f64(3.28084)
///     }
/// }
/// ```
#[macro_export]
macro_rules! unit_type {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default)]
        $vis struct $unit;

        impl<S: [const] $crate::Scalar> const core::ops::Mul<S> for $unit {
            type Output = $crate::quantity::Quantity<$dimension, S>;
            fn mul(self, rhs: S) -> Self::Output {
                $crate::quantity::Quantity::<_, S>::from_scalar::<$unit>(rhs)
            }
        }

        $crate::__unit_mult_imp!(
            $unit, $dimension,
            f64, f32,
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        );

        impl $unit {
            #[inline]
            pub const fn from_scalar<S: [const] $crate::Scalar>(
                value: S,
            ) -> $crate::quantity::Quantity<$dimension, S> {
                <$crate::quantity::Quantity<$dimension, S>>::from_scalar::<Self>(value)
            }
        }
    };
}

/// Creates a unit with a simple linear conversion to the canonical unit.
///
/// Used internally by [`dimension!`]. The conversion factor is expressed as:
/// - `N per canonical`: one canonical unit equals `N` of this unit (e.g. `3.28084 per canonical` for feet)
/// - `per N canonical`: `N` canonical units equal one of this unit
///
/// # Examples
///
/// ```
/// use tuant::premade::Length;
///
/// tuant::simple_unit!(
///     /// A unit of length.
///     pub Feet of dimension Length = 3.28084 per canonical
/// );
/// ```
#[macro_export]
macro_rules! simple_unit {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident = $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?
    ) => {
        $crate::unit_type!(
            $(#[$meta])*
            $vis $unit of dimension $dimension
        );

        impl const $crate::UnitOf<$dimension> for $unit {
            fn convert_to_canonical<S: [const] $crate::Scalar>(canonical: S) -> S {
                $(canonical * S::from_f64($rhsper))? $(canonical / S::from_f64($lhsper))?
            }
            fn convert_from_canonical<S: [const] $crate::Scalar>(canonical: S) -> S {
                $(canonical / S::from_f64($rhsper))? $(canonical * S::from_f64($lhsper))?
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dimension_raw {
    (
        $(#[$meta:meta])*
        $vis:vis dim $name:ident {
            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        $vis enum $name {}

        impl $crate::dimension::Dimension for $name {}

        $(
            $crate::simple_unit!(
                $(#[$unit_meta])*
                $vis $unit of dimension $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )+
    }
}

/// Defines a dimension together with its units and optional cross-dimension conversions.
///
/// # Syntax
///
/// ```text
/// dimension! {
///     pub dim Length(Meters) {           // dimension name + canonical unit
///         Meters: 1.0 per canonical,     // unit: conversion factor
///         Feet:   3.28084 per canonical,
///     } where {                          // optional cross-dimension arithmetic
///         Self / Time => LinearVelocity in MetersPerSecond,
///     }
/// }
/// ```
///
/// Each unit line accepts either `N per canonical` or `per N canonical`:
/// - `3.28084 per canonical`: 1 canonical metre = 3.28084 feet
/// - `per 3.28084 canonical`: equivalent inverse form
#[macro_export]
macro_rules! dimension {
    (
        $(#[$meta:meta])*
        $vis:vis dim $name:ident {
            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )+
        }
    ) => {
        $crate::__dimension_raw!(
            $(#[$meta])*
            $vis dim $name {
                $(
                    $(#[$unit_meta])*
                    $unit: $($rhsper per canonical)? $(per $lhsper canonical)?,
                )+
            }
        );

        impl<D: $crate::dimension::Dimension> $crate::dimension::DimMul<D> for $name {
            type Output = $crate::dimension::Mul<Self, D>;
        }
        impl<D: $crate::dimension::Dimension> $crate::dimension::DimDiv<D> for $name {
            type Output = $crate::dimension::Per<Self, D>;
        }
    };
}

#[macro_export]
macro_rules! derived_dimension {
    {
        $(#[$meta:meta])*
        $vis:vis dim $name:ident($type:ty) {
            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )+
        }
    } => {
        $(#[$meta])*
        $vis type $name = $type;

        $(
            $crate::simple_unit!(
                $(#[$unit_meta])*
                $vis $unit of dimension $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )+
    }
}
