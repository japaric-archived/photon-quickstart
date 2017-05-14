//! Blinks the user LED

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{LED, PinMode};

const PERIOD: u32 = 250; // ms

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    LED.pin_mode(PinMode::Output);
}

fn loop_() {
    LED.low();
    hal::sleep_ms(PERIOD);
    LED.high();
    hal::sleep_ms(PERIOD);
}
