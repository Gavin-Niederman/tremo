#![feature(const_trait_impl, const_ops)]

use tuant::{
    dimension::Mul,
    premade::{Amount, LinearVelocity, Meters, MetersPerSecond, Moles, Seconds},
    quantity::Quantity,
};

fn main() {
    let velocity = const_operations();
    println!("{}", velocity.to::<MetersPerSecond>());

    let num: Quantity<Mul<Amount, LinearVelocity>> = Quantity::from_canonical(1.0f32);
    let den: Quantity<Mul<Amount, LinearVelocity>> = Quantity::from_canonical(1.0f32);

    let simplified = (num / den).simplify();

    let distance = (velocity * (2.0f32 * Seconds)).mul(4.0f32 * Moles) / (1.0f32 * Moles);
    let distance = distance.simplify();
    println!("{distance:?} {simplified:?}")
}

const fn const_operations() -> Quantity<LinearVelocity> {
    let distance = Meters * 1.0;
    let time = Seconds * 2.0;

    distance / time
}
