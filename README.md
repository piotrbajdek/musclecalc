# MUSCLECALC

[musclecalc](https://github.com/piotrbajdek/musclecalc) utilises mathematical formulae from Mayhew et al. (1992) and Wathen (1994) to estimate the maximum weight that can be lifted in a single repetition of a physical exercise, known as the one repetition maximum (1RM). The calculations take into account the body mass, exercise type, and maximum number of repetitions that can be performed without additional weight.

[keywords: bodybuilding, calculator, calisthenics, callisthenics, fitness, gym, muscle strength, physical training, street workout]

Designed for use by both females and males, musclecalc utilises the following models:

**1. Pull-ups**, including chin-ups, dips, and other upper-body exercises that involve lifting the whole body. The Mayhew et al. (1992) equation is used, as it is possibly more accurate for the upper body (LeSuer et al. 1997; Jiménez and De Paz 2008).

**2. Push-ups**, for which the a modified Mayhew et al. (1992) formula is used, assuming that 32.5% of the body weight is supported on the ground by legs when push-ups are performed with a typical position of hands at the height of the chest.

**3. Squats**, for which the Wathen (1994) equation is used, as it is possibly more accurate for the lower body (LeSuer et al. 1997).

**References:**

Jiménez, A. and De Paz, J.A. 2008. Application of the 1RM estimation formulas from the RM in bench press in a group of physically active middle-aged women. _J. Hum. Sport Exerc._ 3 (1): 10–22.

LeSuer, D.A, McCormick, J.H., Mayhew, J.L., Wasserstein, R.L. and Arnold, M.D. 1997. The accuracy of prediction equations for estimating 1-RM performance in the bench press, squat, and deadlift. _J. Strength and Cond. Res._ 11 (4): 211–213.

Mayhew, J.L., Ball, T.E., Arnold, M.D., and Bowen, J.C. 1992. Relative muscular endurance performance as a predictor of bench press strength in college men and women. _J. Appl. Sports Sci. Res._ 6 (4): 200–206.

Wathen, D. 1994. Load assignment. In: T.R. Baechle (Ed.), _Essentials of strength training and conditioning_, pp. 435–446. Champaign, IL: Human Kinetics.

# USAGE

![help-image](https://github.com/piotrbajdek/musclecalc/blob/main/docs/images/help-image.png?raw=true)

![models-image](https://github.com/piotrbajdek/musclecalc/blob/main/docs/images/models-image.png?raw=true)

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/musclecalc/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

[musclecalc](https://github.com/piotrbajdek/musclecalc) is designed to be compatible with **Windows** and **macOS**, and can be easily installed using [cargo](https://www.rust-lang.org/tools/install). However, the primary development and testing environment for musclecalc is **Fedora Linux**.

The current version of musclecalc (v0.2.4) has been verified to work properly on Fedora Linux 37 and Ubuntu 22.10.

## METHOD 1 – USING CARGO

**[Recommended for programmers]**

**1.** To install musclecalc from [crates.io](https://crates.io/crates/musclecalc), use the following cargo command:

_cargo install musclecalc_

The executable will be saved in the hidden `.cargo/bin/` directory within your home directory.

**2a.** For easy access, you may want to copy the musclecalc file to the `/usr/bin/` directory. This can be done by following the instructions in Method 2 (3a, 3b).

**2b.** As an alternative, you can add the `~/.cargo/bin/` directory to your system's PATH variable, which can be configured using [rustup](https://www.rust-lang.org/tools/install).

## METHOD 2 – UNIVERSAL LINUX BINARIES

**1.** To install musclecalc, first download the distro-independent [binary](https://github.com/piotrbajdek/musclecalc/releases/download/v0.2.4/musclecalc) from GitHub.

**2.** Then, make the file executable by running the command:

_sudo chmod +x ./musclecalc_

**3a.** On most Linux distributions, install musclecalc by copying the binary to `/usr/bin/`:

_sudo cp musclecalc /usr/bin/_

**3b.** For Fedora Silverblue / Kinoite, use this command:

_sudo cp musclecalc /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[Recommended for most users]**

Distro-specific packages for [.rpm](https://github.com/piotrbajdek/musclecalc/releases/download/v0.2.4/musclecalc-0.2.4-1.x86_64.rpm) and [.deb](https://github.com/piotrbajdek/musclecalc/releases/download/v0.2.4/musclecalc_0.2.4_amd64.deb)-based Linux distributions are also available for download. To install musclecalc on different Linux distributions, follow these instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i musclecalc-0.2.4-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install musclecalc-0.2.4-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i musclecalc_0.2.4_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

First, download and unpack the musclecalc [source code](https://github.com/piotrbajdek/musclecalc/archive/refs/tags/v0.2.4.zip) from GitHub. Next, to build and install the program, use the command:

_cargo build \--release && sudo cp target/release/musclecalc /usr/bin/_
