#![recursion_limit = "2048"]
#![feature(const_trait_impl, const_ops)]

use tuant::{
    dimension::{CommutedMarker, DOverDMarker, InnerSimplifiableNum, Mul, Per},
    premade::{
        Amount, Energy, Joules, LinearVelocity, Meters, MetersPerSecond, Moles, Seconds, Time,
    },
    quantity::Quantity,
};

fn main() {
    let velocity = const_operations();
    println!("{}", velocity.to::<MetersPerSecond>());

    let unit = (1.0f32 * Joules) / (1.0f32 * Seconds);

    let unit = unit.simplify();
}

const fn const_operations() -> Quantity<LinearVelocity> {
    let distance = Meters * 1.0;
    let time = Seconds * 2.0;

    distance / time
}
