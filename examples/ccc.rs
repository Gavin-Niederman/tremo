#![feature(const_trait_impl, const_ops)]

use tremo::{
    derived_dimension, dimension,
    dimension::{Mul, Per},
    quantity::Quantity,
};

dimension! {
    pub dim LinearVelocity in C {
        C("C"): 1.0 per canonical,
        MetersPerSecond("m/s"): per 299792458.0 canonical,
    }
}

dimension! {
    pub dim Energy in Calories {
        Calories("Cal"): 1.0 per canonical,
        Joules("J"): per 4184.0 canonical,
    }
}

dimension! {
    pub dim Frequency in C4 {
        C4("C4"): 1.0 per canonical,
        Hertz("Hz"): per 261.625565300599 canonical,
    }
}

derived_dimension! {
    pub dim Length(Per<LinearVelocity, Frequency>) {
        CPerC4("C/C4"): 1.0 per canonical,
        Meters("m"): per 1145883.65114682 canonical,
    }
}

derived_dimension! {
    pub dim Mass(Per<Energy, Mul<LinearVelocity, LinearVelocity>>) {
        CaloriePerCSquared("Cal/C²"): 1.0 per canonical,
        Kilograms("kg"): per 4.65532783452834e-14 canonical,
    }
}

fn main() {
    let frequency = 10.0f32 * C4;
    let velocity = 0.01f32 * C;

    let distance: Quantity<Length> = velocity / frequency;

    println!("{}C/C4", distance.to::<CPerC4>());
}
