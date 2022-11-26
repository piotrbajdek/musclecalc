// MUSCLECALC VERSION 0.2.3 / APACHE LICENSE 2.0 © 2022 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod calc;
pub mod menu;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let yellow = "\x1b[93m";
    let red = "\x1b[31m";
    let blue_underlined = "\x1b[34;4m";
    let cyan = "\x1b[36m";
    let grey = "\x1b[38;5;240m";

    menu::documentation(reset, yellow, blue_underlined, cyan, grey);
    calc::list(reset, red, yellow, grey);
}
