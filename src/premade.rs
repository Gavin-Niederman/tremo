use crate::{
    Scalar, UnitOf, derived_dimension, dimension,
    dimension::{Dimensionless, Mul, Per},
    unit_type,
};

// # Base Dimensions

dimension! {
    /// Represents a distance. Canonically represented in meters.
    pub dim Length {
        Millimeters: 1000.0 per canonical,
        Centimeters: 100.0 per canonical,
        Meters: 1.0 per canonical,
        Kilometers: per 1000.0 canonical,
        Inches: per 0.0254 canonical,
        Feet: per 0.3048 canonical,
        Yards: per 0.9144 canonical,
        Miles: per 1609.344 canonical,
        NauticalMiles: per 1852.0 canonical,
    }
}

dimension! {
    pub dim Area {
        SquareMillimeters: 1_000_000.0 per canonical,
        SquareCentimeters: 10_000.0 per canonical,
        SquareMeters: 1.0 per canonical,
        SquareKilometers: per 1_000_000.0 canonical,
        SquareInches: 1550.0031000062 per canonical,
        SquareFeet: 10.7639104167097 per canonical,
        SquareYards: per 0.83612736 canonical,
        Acres: per 4046.8564224 canonical,
    }
}

dimension! {
    pub dim Volume {
        Milliliters: 1_000_000.0 per canonical,
        Liters: 1000.0 per canonical,
        CubicMillimeters: 1_000_000_000.0 per canonical,
        CubicCentimeters: 1_000_000.0 per canonical,
        CubicMeters: 1.0 per canonical,
        CubicKilometers: per 1_000_000_000.0 canonical,
        CubicInches: 61023.7440947323 per canonical,
        CubicFeet: 35.3146667214886 per canonical,
        CubicYards: 1.30795061931439 per canonical,
        FluidOunces: 33814.022701843 per canonical,
        Pints: 2113.37641886519 per canonical,
        Quarts: 1056.68820943259 per canonical,
        Gallons: 264.172052358148 per canonical,
    }
}

dimension! {
    /// Represents a length of time. Canonically represented in seconds.
    pub dim Time {
        Microseconds: 1_000_000.0 per canonical,
        Milliseconds: 1000.0 per canonical,
        Seconds: 1.0 per canonical,
        Minutes: per 60.0 canonical,
        Hours: per 3600.0 canonical,
        Days: per 86_400.0 canonical,
        Weeks: per 604_800.0 canonical,
        Years: per 31_556_926.0 canonical,
    }
}

dimension! {
    pub dim Angle {
        Radians: 1.0 per canonical,
        Rotations: per 6.28318530717959 canonical,
        Degrees: 57.2957795130823 per canonical,
        Gradians: 63.6619772367581 per canonical,
    }
}

dimension! {
    /// Represents mass. Canonically represented in kilograms.
    pub dim Mass {
        Micrograms: 1_000_000_000.0 per canonical,
        Milligrams: 1_000_000.0 per canonical,
        Grams: 1_000.0 per canonical,
        Kilograms: 1.0 per canonical,
        Pounds: per 0.45359237 canonical,
        Ounces: 35.2739619495804 per canonical,
        Stones: per 6.35029318 canonical,
        MetricTons: per 1000.0 canonical,
        ShortTons: per 907.18474 canonical,
        LongTons: per 1016.0469088 canonical,
    }
}

dimension! {
    pub dim Current {
        Milliamperes: 1000.0 per canonical,
        Amperes: 1.0 per canonical,
        Kiloamperes: per 1000.0 canonical,
    }
}

dimension! {
    pub dim Amount {
        Moles: 1.0 per canonical,
    }
}

dimension! {
    pub dim Luminosity {
        Candelas: 1.0 per canonical,
    }
}

dimension! {
    /// Represents temperature. Canonically represented in kelvin.
    pub dim Temperature {
        Kelvin: 1.0 per canonical,
    }
}

unit_type!(pub Celsius of dimension Temperature);
impl const UnitOf<Temperature> for Celsius {
    fn convert_to_canonical<S: [const] Scalar>(value: S) -> S {
        value + S::from_f64(273.15)
    }
    fn convert_from_canonical<S: [const] Scalar>(canonical: S) -> S {
        canonical - S::from_f64(273.15)
    }
}

unit_type!(pub Fahrenheit of dimension Temperature);
impl const UnitOf<Temperature> for Fahrenheit {
    fn convert_to_canonical<S: [const] Scalar>(value: S) -> S {
        (value - S::from_f64(32.0)) * S::from_f64(5.0) / S::from_f64(9.0) + S::from_f64(273.15)
    }
    fn convert_from_canonical<S: [const] Scalar>(canonical: S) -> S {
        (canonical - S::from_f64(273.15)) * S::from_f64(9.0) / S::from_f64(5.0) + S::from_f64(32.0)
    }
}

// # Derived Dimensions

derived_dimension! {
    pub dim LinearVelocity(Per<Length, Time>) {
        MetersPerSecond: 1.0 per canonical,
        KilometersPerSecond: per 1000.0 canonical,
        KilometersPerHour: 3.6 per canonical,
        FeetPerSecond: 3.281 per canonical,
        MilesPerHour: per 0.44704 canonical,
    }
}

derived_dimension! {
    pub dim LinearAcceleration(Per<LinearVelocity, Time>) {
        MetersPerSecondSquared: 1.0 per canonical,
        FeetPerSecondSquared: per 0.3048 canonical,
    }
}

derived_dimension! {
    pub dim AngularVelocity(Per<Angle, Time>) {
        RadiansPerSecond: 1.0 per canonical,
        RotationsPerSecond: per 6.28318530717959 canonical,
        RotationsPerMinute: 9.54929658551372 per canonical,
        DegreesPerSecond: 57.2957795130823 per canonical,
    }
}

derived_dimension! {
    pub dim AngularAcceleration(Per<AngularVelocity, Time>) {
        RadiansPerSecondSquared: 1.0 per canonical,
        RotationsPerSecondSquared: per 6.28318530717959 canonical,
        RotationsPerMinuteSquared: 572.957795130823 per canonical,
        DegreesPerSecondSquared: 57.2957795130823 per canonical,
    }
}

derived_dimension! {
    /// Represents force. Canonically represented in newtons.
    pub dim Force(Mul<Mass, LinearAcceleration>) {
        Newtons: 1.0 per canonical,
        PoundsForce: 4.4482216 per canonical,
        Dynes: per 1e-05 canonical,
    }
}

derived_dimension! {
    /// Represents pressure. Canonically represented in pascals.
    pub dim Pressure(Per<Force, Area>) {
        Pascals: 1.0 per canonical,
        Psi: per 6894.75729316836 canonical,
        Atmospheres: per 101325.0 canonical,
        Bars: per 100_000.0 canonical,
    }
}

derived_dimension! {
    /// Represents torque. Canonically represented in newton-meters per radian.
    pub dim Torque(Per<Mul<Force, Length>, Angle>) {
        NewtonMetersPerRadian: 1.0 per canonical,
        NewtonMetersPerDegree: per 57.2957795130823 canonical,
        PoundFeetPerRadian: per 1.3558179483314 canonical,
        PoundFeetPerDegree: per 77.6826462274756 canonical,
        DyneCentimetersPerRadian: 10_000_000.0 per canonical,
    }
}

derived_dimension! {
    /// Represents energy. Canonically represented in joules.
    pub dim Energy(Mul<Force, Length>) {
        Joules: 1.0 per canonical,
        Calories: per 4.184 canonical,
        Kilocalories: per 4184.0 canonical,
        Ergs: 10e-7 per canonical,
        WattHours: per 3600.0 canonical,
    }
}

derived_dimension! {
    /// Represents power. Canonically represented in watts.
    pub dim Power(Per<Energy, Time>) {
        Watts: 1.0 per canonical,
        Horsepower: per 745.69987158227 canonical,
        ErgsPerSecond: 10e-7 per canonical,
        FootPoundsPerMinute: 44.2537289566359 per canonical,
    }
}

derived_dimension! {
    /// Represents voltage. Canonically represented in volts.
    pub dim Charge(Mul<Current, Time>) {
        Coulombs: 1.0 per canonical,
    }
}

derived_dimension! {
    /// Represents voltage. Canonically represented in volts.
    pub dim Voltage(Per<Energy, Charge>) {
        Millivolts: 1000.0 per canonical,
        Volts: 1.0 per canonical,
        Kilovolts: per 1000.0 canonical,
    }
}

derived_dimension! {
    /// Represents frequency
    pub dim Frequency(Per<Dimensionless, Time>) {
        Hertz: 1.0 per canonical,
    }
}
