//! Particle.function API example
//!
//! Turn on the LED with
//!
//! $ particle call $device led on
//!
//! Turn off the LED with
//!
//! $ particle call $device led off

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use photon::{App, Cloud};
use hal::{D7, PinMode, String, cloud};

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    D7.pin_mode(PinMode::Output);

    if cloud::function("led", led).is_ok() {
        D7.high();
    } else {
        D7.low();
    }
}

fn loop_(_: App) {}

extern "C" fn led(command: &String, _: Cloud) -> i32 {
    if &**command == b"on" {
        D7.high();
        1
    } else if &**command == b"off" {
        D7.low();
        0
    } else {
        -1
    }
}
