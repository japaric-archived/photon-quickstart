//! Heartbeat blinking pattern

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{D7, PinMode};

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    D7.pin_mode(PinMode::Output);
}

fn loop_() {
    D7.high();
    hal::sleep_ms(200);

    D7.low();
    hal::sleep_ms(50);

    D7.high();
    hal::sleep_ms(250);

    D7.low();
    hal::sleep_ms(500);
}
