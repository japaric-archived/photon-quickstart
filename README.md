# `photon-quickstart`

> Cargo template for developing [photon] applications

[photon]: https://particle.io

Firmware version: v0.6.2

# Dependencies

- Embedded ARM toolchain. Any recent version should do.
- Rust nightly toolchain. See https://rustup.rs
- Xargo. `$ rustup run nightly cargo install xargo`
- The rust-src component for nightly: `$ rustup default nightly && rustup
  component add rust-src` If you don't want to keep nightly as the default, you
  can revert the change after installing the `rust-src` component.
- `crc32` and `xxd`. Check how your distribution ships these binaries.

# How to use

```
# Instantiate this template
$ git clone https://github.com/japaric/photon-quickstart

$ mv photon-quickstart my-app && cd $_

$ edit Cargo.toml && head $_
authors = ["Jorge Aparicio <jorge@japaric.io>"]
name = "my-app"
version = "0.1.0"

# build an example (or write your application in src/main.rs and build that)
$ xargo build --example blinky --release

$ arm-none-eabi-size target/photon/release/examples/blinky
   text    data     bss     dec     hex filename
   4468       8    1476    5952    1740 target/photon/release/examples/blinky

# convert the output into a flash-able binary
$ sh elf2bin.sh target/photon/release/examples/blinky

# flash the application
$ particle flash $device blinky.bin
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
   Compiling demo v0.1.0 (file:///home/japaric/tmp/demo)
error: could not exec the linker `arm-none-eabi-g++`: No such file or directory (os error 2)
  |
  = note: "arm-none-eabi-g++" (..)
```

Solution: Install the embedded ARM toolchain. Consult your distribution / OS
package manager.

## Used Cargo instead of Xargo

Error message:

```
$ cargo build
   Compiling particle-hal v0.1.0 (https://github.com/japaric/particle-hal#179f8fb8)
error[E0463]: can't find crate for `core`
  |
  = note: the `photon` target may not be installed

error: aborting due to previous error
```

Solution: use Xargo instead of Cargo. That is `xargo build`.

### Forgot to call the `elf2bin.sh` script

Error message:

```
$ particle flash $device target/..
Including:
    target/photon/release/examples/(..)
attempting to flash firmware to your device (..)
Flash device failed.
[object Object]
```

Solution: Call `elf2bin.sh` on the Cargo output and then (`particle`) flash the
output of that script, a `.bin` file.

## Used the stable toolchain

Error message:

```
$ xargo build
   Compiling particle-hal v0.1.0 (https://github.com/japaric/particle-hal#179f8fb8)
error[E0463]: can't find crate for `core`
  |
  = note: the `photon` target may not be installed

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
