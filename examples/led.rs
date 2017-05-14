//! Turns ON the user LED

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{LED, PinMode};

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    LED.pin_mode(PinMode::Output);
    LED.high();
}

fn loop_() {}
