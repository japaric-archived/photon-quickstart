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

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{D7, PinMode, String, cloud};

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    D7.pin_mode(PinMode::Output);

    if cloud::function("led", toggle).is_ok() {
        D7.high();
    } else {
        D7.low();
    }
}

fn loop_() {}

extern "C" fn toggle(command: &String) -> i32 {
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
