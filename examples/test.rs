#![feature(const_trait_impl, const_ops)]

use tuant::{
    dimension::simplify,
    premade::{LinearVelocity, Meters, MetersPerSecond, Moles, Seconds},
    quantity::Quantity,
};

fn main() {
    let velocity = const_operations();
    println!("{}", velocity.to::<MetersPerSecond>());

    let distance = velocity * (2.0f32 * Seconds) * (4.0f32 * Moles) / (1.0f32 * Moles);
    let distance = simplify(distance);
}

const fn const_operations() -> Quantity<LinearVelocity> {
    let distance = Meters * 1.0;
    let time = Seconds * 2.0;

    distance / time
}
