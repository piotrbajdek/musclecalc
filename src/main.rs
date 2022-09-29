// MUSCLECALC VERSION 0.2.2 / APACHE LICENSE 2.0 © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod calc;
pub mod menu;

// MAIN

fn main() {
    menu::documentation();
    calc::list();
}
