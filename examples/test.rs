#![feature(const_trait_impl, const_ops)]

use tuant::{
    dimension::simplify::{Cancel, Commute, Pass, PassL},
    premade::{Joules, LinearVelocity, Meters, MetersPerSecond, Seconds, Time},
    quantity::Quantity,
};

fn main() {
    let velocity = const_operations();
    println!("{}", velocity.to::<MetersPerSecond>());

    let energy = 1.0f32 * Joules;
    let time = 1.0f32 * Seconds;

    let unit = ((energy * time) / (time * energy)) / time;

    let unit = unit.simplify::<PassL<Commute<Cancel>>>();
}

const fn const_operations() -> Quantity<LinearVelocity> {
    let distance = Meters * 1.0;
    let time = Seconds * 2.0;

    distance / time
}
