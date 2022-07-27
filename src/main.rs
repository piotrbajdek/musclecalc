// MUSCLECALC VERSION 0.2.0 / APACHE LICENSE 2.0 © 2022 PIOTR BAJDEK

use std::env;

fn main() {

   let reset = "\x1b[0m";
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let blue_underlined = "\x1b[34;4m";
   let cyan = "\x1b[36m";
   let grey = "\x1b[38;5;240m";

// GET ARGS TO DISPLAY DOCUMENTATION

   for argument in env::args() {

// ABOUT

      if argument == "-a" || argument == "--about" {
      println!("Program:  {}", yellow.to_owned() + "musclecalc" + reset);
      println!("Version:  0.2.0");
      println!("Date:     July 27, 2022");
      println!("Author:   Piotr Bajdek (Poland)");
      println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
      println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/musclecalc" + reset);
      println!("License:  Apache License 2.0 © 2022 Piotr Bajdek");
      return;
      }

// HELP

      if argument == "-h" || argument == "--help" {
      println!("Usage:     {}", yellow.to_owned() + "[BODY MASS IN KG] [EXERCISE] [MAX. REPETITIONS]" + reset);
      println!("");
      println!("Exercises: {}", cyan.to_owned() + "-p" + reset + ", " + cyan + "--pull-ups" + reset + "    Model for pull-ups");
      println!("           {}", cyan.to_owned() + "-P" + reset + ", " + cyan + "--push-ups" + reset + "    Model for push-ups");
      println!("           {}", cyan.to_owned() + "-s" + reset + ", " + cyan + "--squats" + reset + "      Model for squats");
      println!("");
      println!("Example:   {}", yellow.to_owned() + "musclecalc 69.5 --pull-ups 12" + reset);
      println!("");
      println!("See also:  {}", cyan.to_owned() + "-a" + reset + ", " + cyan + "--about" + reset + "       Show contact and program info");
      println!("           {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "        Show this help");
      println!("           {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--license" + reset + "     Show licesing information");
      println!("           {}", cyan.to_owned() + "-m" + reset + ", " + cyan + "--models" + reset + "      Show implemented formulae");
      println!("           {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "     Show program version");
      return;
      }

// LICENSE

      if argument == "-l" || argument == "--license" {
      println!("{}", yellow.to_owned() + "Copyright 2022 Piotr Bajdek" + reset);
      println!("");
      println!(r#"Licensed under the Apache License, Version 2.0 (the "License");"#);
      println!("you may not use this file except in compliance with the License.");
      println!("You may obtain a copy of the License at");
      println!("");
      println!("{}", blue_underlined.to_owned() + "http://www.apache.org/licenses/LICENSE-2.0" + reset);
      println!("");
      println!("Unless required by applicable law or agreed to in writing, software");
      println!(r#"distributed under the License is distributed on an "AS IS" BASIS,"#);
      println!("WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.");
      println!("See the License for the specific language governing permissions and");
      println!("limitations under the License.");
      return;
      }

// MODELS

      if argument == "-m" || argument == "--models" {
      println!("Pull-ups           {}", cyan.to_owned() + "Mayhew et al. (1992) formula:");
      println!("{}", yellow.to_owned() + "100 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
      println!("");
      println!("Push-ups  {}", cyan.to_owned() + "Modified Mayhew et al. (1992) formula:");
      println!("{}", yellow.to_owned() + " 70 * mass / (52.2 + 41.9 * exp (-0.055 * reps))" + reset);
      println!("");
      println!("Squats                    {}", cyan.to_owned() + "Wathen (1994) formula:");
      println!("{}", yellow.to_owned() + "100 * mass / (48.8 + 53.8 * exp (-0.075 * reps))");
      println!("");
      println!("{}", grey.to_owned() + "***" + reset);
      println!("");
      println!("Mayhew, J.L., Ball, T.E., Arnold, M.D. and Bowen, J.C. 1992. Relative muscular");
      println!("endurance performance as a predictor of bench press strength in college men");
      println!("and women. J. Appl. Sports Sci. Res. 6 (4): 200–206.");
      println!("");
      println!("Wathen, D. 1994. Load assignment. In: T.R. Baechle (Ed.), Essentials of");
      println!("strength training and conditioning, pp. 435–446. Champaign, IL: Human Kinetics.");
      return;
      }

// VERSION

      if argument == "-v" || argument == "--version" {
      println!("Version: {}", yellow.to_owned() + "0.2.0" + reset);
      println!("July 27, 2022");
      return;
      }

   }

// GET ARGS FOR CALCULATIONS

   let args: Vec<String> = env::args().collect();

   let input1 = args.get(1).expect(&(red.to_owned() + "Missing 'body mass' value! See: --help" + reset));
   let mass: f32 = input1.parse().expect(&(red.to_owned() + "Incorrect 'body mass' value! Program only processes numbers! See: --help" + reset));

   let input2 = args.get(2).expect(&(red.to_owned() + "Missing 'exercise type' argument! See: --help" + reset));

   let input3 = args.get(3).expect(&(red.to_owned() + "Missing 'repetitions' value! See: --help" + reset));
   let reps: f32 = input3.parse().expect(&(red.to_owned() + "Incorrect 'repetitions' value! Program only processes numbers! See: --help" + reset));

// PULL-UPS

   if input2 == "-p" || input2 == "--pull-ups" {

   let crep = -0.055 * reps;
   let cexp = f32::exp(crep);
   let mexp = 52.2 + 41.9 * cexp;
   let rm = 100 as f32 * mass / mexp;

   print!("Approximate 1RM = {}", yellow.to_owned());
   print!("{:.1}", rm);
   println!("{}", reset.to_owned() + " kg");

   println!("{}", grey.to_owned() + "Max. weight you can load:" + reset);

   print!("1RM - body mass = {}", yellow.to_owned());
   print!("{:.1}", rm - mass);
   println!("{}", reset.to_owned() + " kg");
   return;
   }

// PUSH-UPS

   if input2 == "-P" || input2 == "--push-ups" {

   let massmod = mass * 0.7;
   let crep = -0.055 * reps;
   let cexp = f32::exp(crep);
   let mexp = 52.2 + 41.9 * cexp;
   let rmmod = 100 as f32 * massmod / mexp;

   println!("{}", grey.to_owned() + "Excluding the weight (30%) supported by legs:" + reset);
   print!("Approximate 1RM = {}", yellow.to_owned());
   print!("{:.1}", rmmod);
   println!("{}", reset.to_owned() + " kg");

   println!("{}", grey.to_owned() + "Max. weight you can load:" + reset);

   print!("1RM - mass * 0.7 = {}", yellow.to_owned());
   print!("{:.1}", rmmod - massmod);
   println!("{}", reset.to_owned() + " kg");
   return;
   }

// SQUATS

   if input2 == "-s" || input2 == "--squats" {

   let crep = -0.075 * reps;
   let cexp = f32::exp(crep);
   let mexp = 48.8 + 53.8 * cexp;
   let rm = 100 as f32 * mass / mexp;

   print!("Approximate 1RM = {}", yellow.to_owned());
   print!("{:.1}", rm);
   println!("{}", reset.to_owned() + " kg");

   println!("{}", grey.to_owned() + "Max. weight you can load:" + reset);

   print!("1RM - body mass = {}", yellow.to_owned());
   print!("{:.1}", rm - mass);
   println!("{}", reset.to_owned() + " kg");
   return;
   }

// INVALID 'EXERCISE TYPE' ARGUMENT

   panic!("{}", red.to_owned() + "Invalid 'exercise type' argument provided! See: --help" + reset);

}
