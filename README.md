# Rust for Arduboy

Running Rust on the [Arduboy](https://arduboy.com/) miniature game system.

The most important first.
I didn't create this project from scratch, @Seeker14491 the legend has already done a lot in his old
project [ArduboyRust](https://github.com/Seeker14491/ArduboyRust)

I just updated and completed the project.

### What do i mean by completed?

I provided all the important functions from the [Arduboy2](https://github.com/MLXXXp/Arduboy2) library in a safe Rust
API.

Most of the Arduboy2 funktions can be called the same way like in C.

# How to use this Project

First clone the repo

```bash
git clone https://github.com/ZennDev1337/Rust-for-Arduboy
```

Install [PlatformIO Core](https://docs.platformio.org/en/latest/core/installation/methods/pypi.html) on your system.
I recommend the pip install if you don't want to configure a new path variable for pio.
Otherwise you will know how to do that.

The rust-analyzer will complain about a missing test crate to fix this
add the following rule to the lsp settings :

```json
{
    "rust-analyzer.checkOnSave.allTargets": false
}
```

If your using Visual Studio Code: Create a folder named `.vscode` and a file named `settings.json` inside. Past the
setting above in the new file. (I have excluded my `.vscode` folder because I have many different settings there that
you don't need)

Your game is located in the Project/game folder. This is also the folder you are working in

Inside the game folder you will find a lib.rs file which contains the setup and loop function as you know it from a
normal Arduboy C project.

You can find the Docs here:

-   [arduboy-rust Crate Docs](https://zenndev1337.github.io/Rust-for-Arduboy/)

or you can use the following command to open the arduboy-rust Crate docs locally

```bash
cargo doc -p arduboy-rust --open
```

Most of the time you will use the prelude.
There is listed witch funktionality you will have.

And last but not least there are multiple examples inside of the Examples folder which shows you how to use the
functions.
I will from time to time also upload my projects to the Example folder so you have as much ressurces as possible.

## Usage of the run tool

**Both run scripts** (only works if you use the given folder structure)

requirements:

-   Python3.11 or higher
-   [PlatformIO Core](https://docs.platformio.org/en/latest/core/installation/methods/pypi.html) must be installed
-   The Arduboy must be plugged in
-   You are in the root directory of this project

All builded `.hex` files are saved inside of `arduboy-rust/Wrapper-Project/build/<GAMENAME>.hex` after you uploaded them
to the Arduboy.

To upload your own game to the Arduboy use:

Linux:

```bash
./run <Project_Name>
```

Windows:

```ps1
.\run.bat <Project_Name>
```

## List of all the Example Games:

### ZennDev1337 Games

-   drboy

### Rust Games

-   snake
-   rustacean

### The demo games / tutorials from the official Arduboy forum [Rewritten in Rust]

-   [Make Your Own Arduboy Game: Part 1 - Setting Up Your Computer](https://community.arduboy.com/t/make-your-own-arduboy-game-part-1-setting-up-your-computer/7924/1)
-   [demo2] [Make Your Own Arduboy Game: Part 2 - Printing Text](https://community.arduboy.com/t/make-your-own-arduboy-game-part-2-printing-text/7925)
-   [demo3] [Make Your Own Arduboy Game: Part 3 - Storing Data & Loops](https://community.arduboy.com/t/make-your-own-arduboy-game-part-3-storing-data-loops/7926)
-   [demo4] [Make Your Own Arduboy Game: Part 4 - Questions & Button Input](https://community.arduboy.com/t/make-your-own-arduboy-game-part-4-questions-button-input/7927)
-   [demo5] [Make Your Own Arduboy Game: Part 5 - Your First Game!](https://community.arduboy.com/t/make-your-own-arduboy-game-part-5-your-first-game/7928)
-   [demo6] [Make Your Own Arduboy Game: Part 6 - Graphics!](https://community.arduboy.com/t/make-your-own-arduboy-game-part-6-graphics/7929)
    Link for the [ZennDev1337 Tile Converter](https://zenndev1337.github.io/Rust-for-Arduboy/tile-converter.html)
-   [demo7] [Make Your Own Arduboy Game: Part 7 - Make Pong From Scratch!](https://community.arduboy.com/t/make-your-own-arduboy-game-part-7-make-pong-from-scratch/7930)
-   Prepare for
    demo9 [Make Your Own Arduboy Game: Part 8 - Starting DinoSmasher](https://community.arduboy.com/t/make-your-own-arduboy-game-part-8-starting-dinosmasher/7932)
-   [demo9] [Make Your Own Arduboy Game: Part 9 - Mapping DinoSmasher](https://community.arduboy.com/t/make-your-own-arduboy-game-part-9-mapping-dinosmasher/7931)
-   [eeprom] [Help, I’m struggling with EEPROM!](https://community.arduboy.com/t/help-im-struggling-with-eeprom/7178)
-   [eeprom-byte] [Help, I’m struggling with EEPROM!](https://community.arduboy.com/t/help-im-struggling-with-eeprom/7178)
-   [progmem] Usage of the big 28'000 Bytes flash memory for Bitmaps Sound sequeces and Text.
-   [tone] [ArduboyTonesTest](https://github.com/MLXXXp/ArduboyTones/blob/master/examples/ArduboyTonesTest/ArduboyTonesTest.ino)

## Usage of the FX Chip

requirements:

```
python3 -m pip install pyserial pillow
```

You need to create a fxdata folder in your project directory.

Don't forget to uncomment the `ArduboyFX_Library` line in the `import_config.h` file.

run Commands:

```
|Project_Dir
->fxdata
->src
  ->lib.rs
>cargo.toml
```

You also need a fxdata.txt file in this new folder.

See the examples in `Examples/ArduboyFX`.

More information:

-   [Arduboy-Python-Utilities by MrBlinky](https://github.com/MrBlinky/Arduboy-Python-Utilities)

run Commands:

-   `fxbuild` is used to build the fxdata.dat file.
-   `fxupload` is used to upload the fxdata.dat file.
-   `fxall` is used to build and upload the fxdata.dat file and build and upload the game all in one step.

Linux:

```bash
./run fxbuild <Project_Name>
./run fxupload <Project_Name>
./run fxall <Project_Name>
```

Windows:

```ps1
.\run.bat fxbuild <Project_Name>
.\run.bat fxupload <Project_Name>
.\run.bat fxall <Project_Name>
```

### Convert the fxdata.h file to Rust

[FXdata Converter by ZennDev1337](https://zenndev1337.github.io/Rust-for-Arduboy/fxdata-converter.html)

# Create a new project

In the root of the repo use the command:

(Don't use "-" in the name because of a cargo feature that takes the "-" and mixes sometimes with a "\_". You will have
some weird behavior with the run tool.)

```bash
cargo new --vcs=none --lib ./Project/newproject
```

Then open the Cargo.toml in your new project and add the following dependencies and settings:

```toml
[lib]
crate-type = ["staticlib"]

[dependencies]

arduboy-rust = { path = "../../arduboy-rust" }
```

Next jump in your lib.rs file and add the following:

```rust
#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
}
```

Now the last step. Go in the Cargo.toml in the root directory and add your project:

```toml
[workspace]
members = [
    #{All other Projects...}
    "Project/newproject",
]
resolver = "2"
```

To run and upload your game use the run tool

```bash
# Linux
./run newproject
```

```ps1
# Windows
.\run.bat newproject
```

## Creating and building an Arduboy crate [Outdated]

You can find instructions on how all build steps work in detail
here [ArduboyRust creating and building an arduboy crate](https://github.com/Seeker14491/ArduboyRust#creating-and-building-an-arduboy-crate)

## Linking the static library to the Arduino C++ project [Outdated]

Instructions on how to link all the static libraries in the Arduino C++ project can be found
here [ArduboyRust linking the static library to the arduino c++ project](https://github.com/Seeker14491/ArduboyRust#linking-the-static-library-to-the-arduino-c-project)

## Credits

Thanks to @Seeker14491 who wrote [ArduboyRust](https://github.com/Seeker14491/ArduboyRust), the starting point for me to
create my own project based on this project. (you are a hero <3)

Thanks to @simon-i1-h who wrote [arduboy-hello-rs](https://github.com/simon-i1-h/arduboy-hello-rs), the proof of concept
that inspired me to try Rust on the Arduboy.

## License

You can license it under either one of those licenses:

-   Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Whichever you may prefer.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
