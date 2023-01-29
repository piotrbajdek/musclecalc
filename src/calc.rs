// MUSCLECALC VERSION 0.2.4 / APACHE LICENSE 2.0 © 2022–2023 PIOTR BAJDEK

// MODULE CALC

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::similar_names, clippy::missing_panics_doc)]

// IMPORTS

use std::env;

// CALCULATIONS

pub fn list(reset: &str, grey: &str, red: &str, violet: &str, yellow: &str) {
    // COLLECT ARGUMENTS FOR CALCULATIONS

    let args: Vec<String> = env::args().collect();

    let input1 = args.get(1).expect(&(red.to_owned() + "Missing 'body mass' value! See: --help" + reset));
    let mass: f32 = input1.parse().expect(&(red.to_owned() + "Incorrect 'body mass' value! Program only processes numbers! See: --help" + reset));

    let input2 = args.get(2).expect(&(red.to_owned() + "Missing 'exercise type' argument! See: --help" + reset));

    let input3 = args.get(3).expect(&(red.to_owned() + "Missing 'repetitions' value! See: --help" + reset));
    let reps: f32 = input3.parse().expect(&(red.to_owned() + "Incorrect 'repetitions' value! Program only processes numbers! See: --help" + reset));

    //   ++++++++++   ++++++++++   ++++++++++

    // PULL-UPS

    if input2 == "-p" || input2 == "--pull-ups" {
        let crep = -0.055 * reps;
        let cexp = f32::exp(crep);
        let mexp = 41.9f32.mul_add(cexp, 52.2);
        let rm = 100_f32 * mass / mexp;

        print!("{}", violet.to_owned() + "Approximate 1RM" + reset + " = " + yellow);
        print!("{rm:.1}");
        println!("{}", reset.to_owned() + " kg");

        println!("{}", grey.to_owned() + "Max. weight you can load" + reset + ":");

        print!("{}", violet.to_owned() + "1RM " + reset + "-" + violet + " body mass" + reset + " = " + yellow);
        print!("{:.1}", rm - mass);
        println!("{}", reset.to_owned() + " kg");
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // PUSH-UPS

    if input2 == "-P" || input2 == "--push-ups" {
        let massmod = mass * 0.675;
        let crep = -0.055 * reps;
        let cexp = f32::exp(crep);
        let mexp = 41.9f32.mul_add(cexp, 52.2);
        let rmmod = 100_f32 * massmod / mexp;

        println!("{}", grey.to_owned() + "Excluding the weight supported by legs (32.5%)" + reset + ":");
        print!("{}", violet.to_owned() + "Approximate 1RM" + reset + " = " + yellow);
        print!("{rmmod:.1}");
        println!("{}", reset.to_owned() + " kg");

        println!("{}", grey.to_owned() + "Max. weight you can load" + reset + ":");

        print!("{}", violet.to_owned() + "1RM " + reset + "-" + violet + " mass " + reset + "*" + violet + " 0.675" + reset + " = " + yellow);
        print!("{:.1}", rmmod - massmod);
        println!("{}", reset.to_owned() + " kg");
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // SQUATS

    if input2 == "-s" || input2 == "--squats" {
        let crep = -0.075 * reps;
        let cexp = f32::exp(crep);
        let mexp = 53.8f32.mul_add(cexp, 48.8);
        let rm = 100_f32 * mass / mexp;

        print!("{}", violet.to_owned() + "Approximate 1RM" + reset + " = " + yellow);
        print!("{rm:.1}");
        println!("{}", reset.to_owned() + " kg");

        println!("{}", grey.to_owned() + "Max. weight you can load" + reset + ":");

        print!("{}", violet.to_owned() + "1RM " + reset + "-" + violet + " body mass" + reset + " = " + yellow);
        print!("{:.1}", rm - mass);
        println!("{}", reset.to_owned() + " kg");
        return;
    }

    //   ++++++++++   ++++++++++   ++++++++++

    // INVALID 'EXERCISE TYPE' ARGUMENT

    panic!("{}", red.to_owned() + "Invalid 'exercise type' argument provided! See: --help" + reset);
}
