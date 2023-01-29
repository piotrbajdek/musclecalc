// MUSCLECALC VERSION 0.2.4 / APACHE LICENSE 2.0 © 2022–2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation(reset: &str, blue_underlined: &str, grey: &str, violet: &str, yellow: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("{}", grey.to_owned() + "Program" + reset + ":  " + yellow + "musclecalc" + reset);
            println!("{}", grey.to_owned() + "Version" + reset + ":  0.2.4");
            println!("{}", grey.to_owned() + "Date" + reset + ":     January 29, 2023");
            println!("{}", grey.to_owned() + "Author" + reset + ":   Piotr Bajdek");
            println!("{}", grey.to_owned() + "Contact" + reset + ":  " + blue_underlined + "piotr.bajdek@proton.me" + reset);
            println!("{}", grey.to_owned() + "Source" + reset + ":   " + blue_underlined + "https://github.com/piotrbajdek/musclecalc" + reset);
            println!("{}", grey.to_owned() + "License" + reset + ":  Apache License 2.0 © 2022–2023 Piotr Bajdek");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", grey.to_owned() + "Usage" + reset + ":     " + yellow + "musclecalc [body mass in kg] [exercise] [max. repetitions]" + reset);
            println!();
            println!("{}", grey.to_owned() + "Exercises" + reset + ": " + violet + "-p" + reset + ", " + violet + "--pull-ups" + reset + "    Model for pull-ups");
            println!("           {}", violet.to_owned() + "-P" + reset + ", " + violet + "--push-ups" + reset + "    Model for push-ups");
            println!("           {}", violet.to_owned() + "-s" + reset + ", " + violet + "--squats" + reset + "      Model for squats");
            println!();
            println!("{}", grey.to_owned() + "Example" + reset + ":   " + yellow + "musclecalc 69.5 --pull-ups 12" + reset);
            println!();
            println!("{}", grey.to_owned() + "See also" + reset + ":  " + violet + "-a" + reset + ", " + violet + "--about" + reset + "       Display contact and program information");
            println!("           {}", violet.to_owned() + "-h" + reset + ", " + violet + "--help" + reset + "        Display the help menu");
            println!("           {}", violet.to_owned() + "-l" + reset + ", " + violet + "--license" + reset + "     Display licensing information");
            println!("           {}", violet.to_owned() + "-m" + reset + ", " + violet + "--models" + reset + "      Display the list of implemented formulae");
            println!("           {}", violet.to_owned() + "-v" + reset + ", " + violet + "--version" + reset + "     Display the program version");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "Copyright 2022–2023 Piotr Bajdek" + reset);
            println!();
            println!(r#"Licensed under the Apache License, Version 2.0 (the "License");"#);
            println!("you may not use this file except in compliance with the License.");
            println!("You may obtain a copy of the License at");
            println!();
            println!("{}", blue_underlined.to_owned() + "http://www.apache.org/licenses/LICENSE-2.0" + reset);
            println!();
            println!("Unless required by applicable law or agreed to in writing, software");
            println!(r#"distributed under the License is distributed on an "AS IS" BASIS,"#);
            println!("WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.");
            println!("See the License for the specific language governing permissions and");
            println!("limitations under the License.");
            exit(0);
        }

        // MODELS

        if argument == "-m" || argument == "--models" {
            println!("Pull-ups            {}", violet.to_owned() + "Mayhew et al. (1992) formula" + reset + ":");
            println!("{}", yellow.to_owned() + " 100 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
            println!();
            println!("Push-ups   {}", violet.to_owned() + "Modified Mayhew et al. (1992) formula" + reset + ":");
            println!("{}", yellow.to_owned() + "67.5 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
            println!();
            println!("Squats                     {}", violet.to_owned() + "Wathen (1994) formula" + reset + ":");
            println!("{}", yellow.to_owned() + " 100 * mass / (48.8 + 53.8 * exp (-0.075 * reps))");
            println!();
            println!("{}", grey.to_owned() + "References" + reset + ":");
            println!();
            println!("Mayhew, J.L., Ball, T.E., Arnold, M.D. and Bowen, J.C. 1992. Relative muscular");
            println!("endurance performance as a predictor of bench press strength in college men");
            println!("and women. J. Appl. Sports Sci. Res. 6 (4): 200–206.");
            println!();
            println!("Wathen, D. 1994. Load assignment. In: T.R. Baechle (Ed.), Essentials of");
            println!("strength training and conditioning, pp. 435–446. Champaign, IL: Human Kinetics.");
            exit(0);
        }

        // VERSION

        if argument == "-v" || argument == "--version" {
            println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "0.2.4" + reset);
            println!("January 29, 2023");
            exit(0);
        }
    }
}
