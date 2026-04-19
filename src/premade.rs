use crate::dimension;

// # Base Dimensions

dimension! {
    pub dim Length(Meters) {
        Meters: 1.0 per canonical,
    } where {
        Self / Time => LinearVelocity in MetersPerSecond,
        Self * any,
    }
}

dimension! {
    pub dim Time(Seconds) {
        Seconds: 1.0 per canonical,
    }
}

dimension! {
    pub dim Angle(Radians) {
        Radians: 1.0 per canonical,
    }
}

dimension! {
    pub dim Mass(Kilograms) {
        Kilograms: 1.0 per canonical,
    }
}

dimension! {
    pub dim Current(Amperes) {
        Amperes: 1.0 per canonical,
    }
}

dimension! {
    pub dim Temperature(Kelvin) {
        Kelvin: 1.0 per canonical,
    }
}

dimension! {
    pub dim Amount(Mole) {
        Moles: 1.0 per canonical,
    }
}

dimension! {
    pub dim Luminosity(Candela) {
        Candelas: 1.0 per canonical,
    }
}

// # Derived Dimensions

// ## Linear

dimension! {
    pub dim LinearVelocity(MetersPerSecond) {
        MetersPerSecond: 1.0 per canonical,
    } where {
        Self * Time => Length in Meters,
    }
}

// pub type LinearVelocity = Div<Length, Time>;
// pub type LinearAcceleration = Div<Div<Length, Time>, Time>;
// pub type LinearJerk = Div<Div<Div<Length, Time>, Time>, Time>;

// pub type Area = Mul<Length, Length>;
// pub type Volume = Mul<Mul<Length, Length>, Length>;

// pub type AngularVelocity = Div<Angle, Time>;
// pub type AngularAcceleration = Div<Div<Angle, Time>, Time>;
// pub type AngularJerk = Div<Div<Div<Angle, Time>, Time>, Time>;

// pub type Frequency = Div<Dimensionless, Time>;
