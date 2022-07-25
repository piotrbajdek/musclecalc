# OVERVIEW OF MUSCLECALC

[musclecalc](https://github.com/piotrbajdek/musclecalc) employs mathematical formulae from Mayhew et al. (1992) and Wathen (1994) to estimate the maximum weight one can lift in a single repetition of a physical exercise (known as 1RM: one repetition maximum). Calculations are based on (1) the body mass, (2) the exercise type, and (3) the maximum number of repetitions one is able to perform in a single set without an additional weight (i.e. with its own body mass).

[keywords: bodybuilding, calisthenics, callisthenics, fitness, gym, muscle strength, street workout]

musclecalc can be used by both females and males. Implemented models:

**1. Pull-ups** (including chin-ups, will also work fine for dips and other upper-body exercises lifting the whole body)

– Mayhew et al. (1992) equation is used as it appears to be more accurate for the upper body (LeSuer et al. 1997; Jiménez and De Paz 2008).

**2. Squats**

– Wathen (1994) equation is used as it appears to be more accurate for the lower body (LeSuer et al. 1997).

musclecalc remains at an early development stage and more functionalities may be implemented soon.

**References:**

Jiménez, A. and De Paz, J.A. 2008. Application of the 1RM estimation formulas from the RM in bench press in a group of physically active middle-aged women. _J. Hum. Sport Exerc._ 3 (1): 10–22.

LeSuer, D.A, McCormick, J.H., Mayhew, J.L., Wasserstein, R.L. and Arnold, M.D. 1997. The accuracy of prediction equations for estimating 1-RM performance in the bench press, squat, and deadlift. _J. Strength and Cond. Res._ 11 (4): 211–213.

Mayhew, J.L., Ball, T.E., Arnold, M.D., and Bowen, J.C. 1992. Relative muscular endurance performance as a predictor of bench press strength in college men and women. _J. Appl. Sports Sci. Res._ 6 (4): 200–206.

Wathen, D. 1994. Load assignment. In: T.R. Baechle (Ed.), _Essentials of strength training and conditioning_, pp. 435–446. Champaign, IL: Human Kinetics.

# USAGE

[Static links to changeable images of the _most recent_ version of musclecalc! This may include pre-releases!]

![help-image](https://github.com/piotrbajdek/musclecalc/blob/main/docs/images/help-image.png?raw=true)

![example-image-1](https://github.com/piotrbajdek/musclecalc/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

[musclecalc](https://github.com/piotrbajdek/musclecalc) should run smoothly on Windows and macOS, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and tested on Fedora Linux.

## METHOD 1

**1.** Install from crates.io by the use of cargo:

_cargo install musclecalc_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy musclecalc to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (see documentation of your shell).

## METHOD 2

**1.** Download the binary 'musclecalc' for Linux x86_64 from GitHub:

https://github.com/piotrbajdek/musclecalc/releases/tag/v0.1.0

**2.** Make the file executable:

_sudo chmod +x ./musclecalc_

**3a.** Install musclecalc via copying the binary to `/usr/bin/` on most Linux distros:

_sudo cp musclecalc /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp musclecalc /var/usrlocal/bin/_

## METHOD 3

Download and unpack the musclecalc source from GitHub. Then, build and install the program:

https://github.com/piotrbajdek/musclecalc/releases/tag/v0.1.0

_cargo build \--release && sudo cp target/release/musclecalc /usr/bin/_

# MUSCLECALC CRATE ON CRATES.IO

The Rust community’s crate registry

https://crates.io/crates/musclecalc
