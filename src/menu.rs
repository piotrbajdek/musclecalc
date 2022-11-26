// MUSCLECALC VERSION 0.2.3 / APACHE LICENSE 2.0 © 2022 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation(reset: &str, yellow: &str, blue_underlined: &str, cyan: &str, grey: &str) {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("Program:  {}", yellow.to_owned() + "musclecalc" + reset);
            println!("Version:  0.2.3");
            println!("Date:     November 26, 2022");
            println!("Author:   Piotr Bajdek (Poland)");
            println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
            println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/musclecalc" + reset);
            println!("License:  Apache License 2.0 © 2022 Piotr Bajdek");
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("Usage:     {}", yellow.to_owned() + "[BODY MASS IN KG] [EXERCISE] [MAX. REPETITIONS]" + reset);
            println!();
            println!("Exercises: {}", cyan.to_owned() + "-p" + reset + ", " + cyan + "--pull-ups" + reset + "    Model for pull-ups");
            println!("           {}", cyan.to_owned() + "-P" + reset + ", " + cyan + "--push-ups" + reset + "    Model for push-ups");
            println!("           {}", cyan.to_owned() + "-s" + reset + ", " + cyan + "--squats" + reset + "      Model for squats");
            println!();
            println!("Example:   {}", yellow.to_owned() + "musclecalc 69.5 --pull-ups 12" + reset);
            println!();
            println!("See also:  {}", cyan.to_owned() + "-a" + reset + ", " + cyan + "--about" + reset + "       Show contact and program info");
            println!("           {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "        Show this help");
            println!("           {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--license" + reset + "     Show licensing information");
            println!("           {}", cyan.to_owned() + "-m" + reset + ", " + cyan + "--models" + reset + "      Show implemented formulae");
            println!("           {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "     Show program version");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "Copyright 2022 Piotr Bajdek" + reset);
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
            println!("Pull-ups            {}", cyan.to_owned() + "Mayhew et al. (1992) formula:");
            println!("{}", yellow.to_owned() + " 100 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
            println!();
            println!("Push-ups   {}", cyan.to_owned() + "Modified Mayhew et al. (1992) formula:");
            println!("{}", yellow.to_owned() + "67.5 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
            println!();
            println!("Squats                     {}", cyan.to_owned() + "Wathen (1994) formula:");
            println!("{}", yellow.to_owned() + " 100 * mass / (48.8 + 53.8 * exp (-0.075 * reps))");
            println!();
            println!("{}", grey.to_owned() + "***" + reset);
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
            println!("Version: {}", yellow.to_owned() + "0.2.3" + reset);
            println!("November 26, 2022");
            exit(0);
        }
    }
}
