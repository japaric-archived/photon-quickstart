//! Turns ON the user LED

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use hal::{LED, PinMode};
use photon::App;

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    LED.pin_mode(PinMode::Output);
    LED.high();
}

fn loop_(_: App) {}
