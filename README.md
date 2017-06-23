# `photon-quickstart`

> Cargo template for developing [photon] applications

[photon]: https://particle.io

Firmware version: v0.6.2

# Dependencies

- Embedded ARM toolchain. Any recent version should do.
  - On Windows, check
    the [GNU ARM Embedded Toolchain](https://launchpad.net/gcc-arm-embedded).
    The download links are on the left side.
  - On macOS, run `brew cask install gcc-arm-embedded`.
  - On Ubuntu, install the `gcc-arm-none-eabi`, `libnewlib-arm-none-eabi` and
    `libstdc++-arm-none-eabi-newlib` packages.
- Rust nightly toolchain. See https://rustup.rs for installation instructions,
  or run `rustup default nightly` (or create an override) if you already have
  rustup installed.
- Xargo. `$ cargo install xargo`
- The rust-src component for the nightly channel. `$ rustup component add
  rust-src`.
- `particle-cli`. `$ npm -g install particle-cli`.
- `particle-tools`, for the `elf2bin` tool. `$ cargo install --git
  https://github.com/japaric/particle-tools`

# How to use

``` console
$ # Grab a copy of this template
$ git clone https://github.com/japaric/photon-quickstart my-app && cd $_

$ # rename the project
$ edit Cargo.toml && head $_
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "my-app"
version = "0.1.0"

$ # build an example (or write your application in src/main.rs and build that)
$ xargo build --example function
(..)
   Compiling my-app v0.1.0 (file:///home/japaric/tmp/my-app)
    Finished dev [optimized] target(s) in 0.32 secs

$ arm-none-eabi-size target/thumbv7m-none-eabi/debug/examples/function
   text    data     bss     dec     hex filename
   4628       8    1476    6112    17e0 target/thumbv7m-none-eabi/debug/examples/function

$ # convert the output into a flash-able binary
$ elf2bin target/thumbv7m-none-eabi/debug/examples/function

$ # flash the application, the D7 LED on the board should turn on
$ particle flash $device function.bin
Including:
    function.bin
attempting to flash firmware to your device Ferris
Flash device OK:  Update started

$ # turn off the LED
$ particle call $device led off
0

$ # turn it on again
$ particle call $device led on
1
```

# Troubleshooting

This section contains fixes for common errors encountered when using this
template.

### Forgot to install the `rust-src` component

Old error message (Xargo <=v0.3.6):

```
$ xargo build
error: couldn't walk the sysroot
caused by: IO error for operation on /home/japaric/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src: No such file or directory (os error 2)
caused by: No such file or directory (os error 2)
note: run with `RUST_BACKTRACE=1` for a backtrace
```

New error message:

```
$ xargo build
error: `rust-src` component not found. Run `rustup component add rust-src`.
note: run with `RUST_BACKTRACE=1` for a backtrace
```

Solution: Install the `rust-src` component using the command `rustup component
add rust-src`.

## Forgot to install the `arm-none-eabi-g++` linker

Error message:

```
$ xargo build
   Compiling my-app v0.1.0 (file:///home/japaric/tmp/my-app)
error: could not exec the linker `arm-none-eabi-g++`: No such file or directory (os error 2)
  |
  = note: "arm-none-eabi-g++" (..)
```

Solution: Install the embedded ARM toolchain. Consult your distribution / OS
package manager.

## Didn't install the `libnewlib-arm-none-eabi` package

Error message:

```
$ xargo build
   Compiling my-app v0.1.0 (file:///home/japaric/tmp/my-app)
error: linking with `arm-none-eabi-g++` failed: exit code: 1
  |
  = note: "arm-none-eabi-g++" (..)
  = note: arm-none-eabi-g++: error: nano.specs: No such file or directory
```

Solution: Install the `libnewlib-arm-none-eabi` package. This is on Ubuntu;
other distributions may call the package with a different name.

## Didn't install the `libstdc++-arm-none-eabi-newlib` package

Error message:

```
$ xargo build
   Compiling my-app v0.1.0 (file:///home/japaric/tmp/my-app)
error: linking with `arm-none-eabi-g++` failed: exit code: 1
  |
  = note: "arm-none-eabi-g++" (..)
  = note: /usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/bin/ld: cannot find -lstdc++_nano
```

Solution: Install the `libstdc++-arm-none-eabi-newlib` package. This is on
Ubuntu; other distributions may call the package with a different name.

## Used Cargo instead of Xargo

Error message:

```
$ cargo build
   Compiling particle-hal v0.1.0 (https://github.com/japaric/particle-hal#179f8fb8)
error[E0463]: can't find crate for `core`
  |
  = note: the `thumbv7m-none-eabi` target may not be installed

error: aborting due to previous error
```

Solution: use Xargo instead of Cargo. That is `xargo build`.

### Forgot to call the `elf2bin` tool

Error message:

```
$ particle flash $device target/..
Including:
    target/thumbv7m-none-eabi/release/examples/(..)
attempting to flash firmware to your device (..)
Flash device failed.
[object Object]
```

Solution: Call `elf2bin` on the Cargo output and then (`particle`) flash the
output of that script: a `.bin` file.

## Used the stable toolchain

Error message:

```
$ xargo build
   Compiling particle-hal v0.1.0 (https://github.com/japaric/particle-hal#179f8fb8)
error[E0463]: can't find crate for `core`
  |
  = note: the `thumbv7m-none-eabi` target may not be installed

error: aborting due to previous error
```

Solution: Switch to the nightly toolchain using the command `rustup default
nightly`.

# License

The Rust code in repository is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The binary blobs in this repository were generated from the [spark/firmware]
repository and as such they are licensed according to [their terms].

[spark/firmware]: https://github.com/spark/firmware/tree/v0.6.2
[their terms]: https://github.com/spark/firmware/tree/v0.6.2#license

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
