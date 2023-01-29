// MUSCLECALC VERSION 0.2.4 / APACHE LICENSE 2.0 © 2022–2023 PIOTR BAJDEK

// MAIN FILE

// CLIPPY LINTS

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

// IMPORTS

pub mod calc;
pub mod menu;

// MAIN

fn main() {
    let reset = "\x1b[0m";
    let yellow = "\x1b[38;5;220m";
    let red = "\x1b[31m";
    let blue_underlined = "\x1b[34;4m";
    let violet = "\x1b[38;5;133m";
    let grey = "\x1b[38;5;240m";

    menu::documentation(reset, blue_underlined, grey, violet, yellow);
    calc::list(reset, grey, red, violet, yellow);
}
