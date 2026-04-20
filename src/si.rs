use crate::{
    Scalar, UnitOf, derived_dimension, dimension,
    dimension::{Dimensionless, Mul, Per},
    unit_type,
};

// # Base Dimensions

dimension! {
    /// Represents a distance. Canonically represented in meters.
    pub dim Length in Meters {
        Millimeters("mm"): 1000.0 per canonical,
        Centimeters("cm"): 100.0 per canonical,
        Meters("m"): 1.0 per canonical,
        Kilometers("km"): per 1000.0 canonical,
        Inches("in"): per 0.0254 canonical,
        Feet("ft"): per 0.3048 canonical,
        Yards("yd"): per 0.9144 canonical,
        Miles("mi"): per 1609.344 canonical,
        NauticalMiles("nmi"): per 1852.0 canonical,
    }
}

dimension! {
    /// Represents a length of time. Canonically represented in seconds.
    pub dim Time {
        Microseconds("μs"): 1_000_000.0 per canonical,
        Milliseconds("ms"): 1000.0 per canonical,
        Seconds("s"): 1.0 per canonical,
        Minutes("min"): per 60.0 canonical,
        Hours("h"): per 3600.0 canonical,
        Days("d"): per 86_400.0 canonical,
        Weeks("wk"): per 604_800.0 canonical,
        Years("yr"): per 31_556_926.0 canonical,
    }
}

dimension! {
    pub dim Angle {
        Radians("rad"): 1.0 per canonical,
        Rotations("rot"): per 6.28318530717959 canonical,
        Degrees("°"): 57.2957795130823 per canonical,
        Gradians("grad"): 63.6619772367581 per canonical,
    }
}

dimension! {
    /// Represents mass. Canonically represented in kilograms.
    pub dim Mass {
        Micrograms("μg"): 1_000_000_000.0 per canonical,
        Milligrams("mg"): 1_000_000.0 per canonical,
        Grams("g"): 1_000.0 per canonical,
        Kilograms("kg"): 1.0 per canonical,
        Pounds("lb"): per 0.45359237 canonical,
        Ounces("oz"): 35.2739619495804 per canonical,
        Stones("st"): per 6.35029318 canonical,
        MetricTons("t"): per 1000.0 canonical,
        ShortTons("tn"): per 907.18474 canonical,
        LongTons("LT"): per 1016.0469088 canonical,
    }
}

dimension! {
    pub dim Current {
        Milliamperes("mA"): 1000.0 per canonical,
        Amperes("A"): 1.0 per canonical,
        Kiloamperes("kA"): per 1000.0 canonical,
    }
}

dimension! {
    pub dim Amount {
        Moles("mol"): 1.0 per canonical,
    }
}

dimension! {
    pub dim Luminosity {
        Candelas("cd"): 1.0 per canonical,
    }
}

dimension! {
    /// Represents temperature. Canonically represented in kelvin.
    pub dim Temperature {
        Kelvin("K"): 1.0 per canonical,
    }
}

unit_type!(pub Celsius("C") of dimension Temperature);
impl const UnitOf<Temperature> for Celsius {
    fn convert_to_canonical<S: [const] Scalar>(value: S) -> S {
        value + S::from_f64(273.15)
    }
    fn convert_from_canonical<S: [const] Scalar>(canonical: S) -> S {
        canonical - S::from_f64(273.15)
    }
}

unit_type!(pub Fahrenheit("F") of dimension Temperature);
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
    pub dim Area(Mul<Length, Length>) in SquareMeters {
        SquareMillimeters("mm²"): 1_000_000.0 per canonical,
        SquareCentimeters("cm²"): 10_000.0 per canonical,
        SquareMeters("m²"): 1.0 per canonical,
        SquareKilometers("km²"): per 1_000_000.0 canonical,
        SquareInches("in²"): 1550.0031000062 per canonical,
        SquareFeet("ft²"): 10.7639104167097 per canonical,
        SquareYards("yd²"): per 0.83612736 canonical,
        Acres("ac"): per 4046.8564224 canonical,
    }
}

derived_dimension! {
    pub dim Volume(Mul<Area, Length>) in CubicMeters {
        Milliliters("mL"): 1_000_000.0 per canonical,
        Liters("L"): 1000.0 per canonical,
        CubicMillimeters("mm³"): 1_000_000_000.0 per canonical,
        CubicCentimeters("cm³"): 1_000_000.0 per canonical,
        CubicMeters("m³"): 1.0 per canonical,
        CubicKilometers("km³"): per 1_000_000_000.0 canonical,
        CubicInches("in³"): 61023.7440947323 per canonical,
        CubicFeet("ft³"): 35.3146667214886 per canonical,
        CubicYards("yd³"): 1.30795061931439 per canonical,
        FluidOunces("fl oz"): 33814.022701843 per canonical,
        Pints("pt"): 2113.37641886519 per canonical,
        Quarts("qt"): 1056.68820943259 per canonical,
        Gallons("gal"): 264.172052358148 per canonical,
    }
}

derived_dimension! {
    pub dim LinearVelocity(Per<Length, Time>) in MetersPerSecond {
        MetersPerSecond("m/s"): 1.0 per canonical,
        KilometersPerSecond("km/s"): per 1000.0 canonical,
        KilometersPerHour("km/h"): 3.6 per canonical,
        FeetPerSecond("ft/s"): 3.281 per canonical,
        MilesPerHour("mph"): per 0.44704 canonical,
    }
}

derived_dimension! {
    pub dim LinearAcceleration(Per<LinearVelocity, Time>) in MetersPerSecondSquared {
        MetersPerSecondSquared("m/s²"): 1.0 per canonical,
        FeetPerSecondSquared("ft/s²"): per 0.3048 canonical,
    }
}

derived_dimension! {
    pub dim AngularVelocity(Per<Angle, Time>) in RadiansPerSecond {
        RadiansPerSecond("rad/s"): 1.0 per canonical,
        RotationsPerSecond("rps"): per 6.28318530717959 canonical,
        RotationsPerMinute("rpm"): 9.54929658551372 per canonical,
        DegreesPerSecond("°/s"): 57.2957795130823 per canonical,
    }
}

derived_dimension! {
    pub dim AngularAcceleration(Per<AngularVelocity, Time>) in RadiansPerSecondSquared {
        RadiansPerSecondSquared("rad/s²"): 1.0 per canonical,
        RotationsPerSecondSquared("rps²"): per 6.28318530717959 canonical,
        RotationsPerMinuteSquared("rpm/s"): 572.957795130823 per canonical,
        DegreesPerSecondSquared("°/s²"): 57.2957795130823 per canonical,
    }
}

derived_dimension! {
    /// Represents force. Canonically represented in newtons.
    pub dim Force(Mul<Mass, LinearAcceleration>) in Newtons {
        Newtons("N"): 1.0 per canonical,
        PoundsForce("lbf"): 4.4482216 per canonical,
        Dynes("dyn"): per 1e-05 canonical,
    }
}

derived_dimension! {
    /// Represents pressure. Canonically represented in pascals.
    pub dim Pressure(Per<Force, Area>) in Pascals {
        Pascals("Pa"): 1.0 per canonical,
        Psi("psi"): per 6894.75729316836 canonical,
        Atmospheres("atm"): per 101325.0 canonical,
        Bars("bar"): per 100_000.0 canonical,
    }
}

derived_dimension! {
    /// Represents torque. Canonically represented in newton-meters per radian.
    pub dim Torque(Per<Mul<Force, Length>, Angle>) in NewtonMetersPerRadian {
        NewtonMetersPerRadian("N·m/rad"): 1.0 per canonical,
        NewtonMetersPerDegree("N·m/°"): per 57.2957795130823 canonical,
        PoundFeetPerRadian("lb·ft/rad"): per 1.3558179483314 canonical,
        PoundFeetPerDegree("lb·ft/°"): per 77.6826462274756 canonical,
        DyneCentimetersPerRadian("dyn·cm/rad"): 10_000_000.0 per canonical,
    }
}

derived_dimension! {
    /// Represents energy. Canonically represented in joules.
    pub dim Energy(Mul<Force, Length>) in Joules {
        Joules("J"): 1.0 per canonical,
        Calories("cal"): per 4.184 canonical,
        Kilocalories("kcal"): per 4184.0 canonical,
        Ergs("erg"): 10e-7 per canonical,
        WattHours("Wh"): per 3600.0 canonical,
    }
}

derived_dimension! {
    /// Represents power. Canonically represented in watts.
    pub dim Power(Per<Energy, Time>) in Watts {
        Watts("W"): 1.0 per canonical,
        Horsepower("hp"): per 745.69987158227 canonical,
        ErgsPerSecond("erg/s"): 10e-7 per canonical,
        FootPoundsPerMinute("ft·lbf/min"): 44.2537289566359 per canonical,
    }
}

derived_dimension! {
    /// Represents charge. Canonically represented in coulombs.
    pub dim Charge(Mul<Current, Time>) in Coulombs {
        Coulombs("C"): 1.0 per canonical,
    }
}

derived_dimension! {
    /// Represents voltage. Canonically represented in volts.
    pub dim Voltage(Per<Energy, Charge>) in Volts {
        Millivolts("mV"): 1000.0 per canonical,
        Volts("V"): 1.0 per canonical,
        Kilovolts("kV"): per 1000.0 canonical,
    }
}

derived_dimension! {
    /// Represents frequency. Canonically represented in hertz.
    pub dim Frequency(Per<Dimensionless, Time>) in Hertz {
        Hertz("Hz"): 1.0 per canonical,
    }
}
