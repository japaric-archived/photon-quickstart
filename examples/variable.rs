//! Example for the Particle.variable API

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{LED, PinMode, cloud};

app! {
    setup: setup,
    loop: loop_,
}

const PERIOD: u32 = 1_000;
static mut COUNTER: i32 = 0;

fn setup() {
    LED.pin_mode(PinMode::Output);

    if cloud::variable("counter", unsafe { &COUNTER }).is_ok() {
        LED.high()
    } else {
        LED.low()
    }
}

fn loop_() {
    LED.high();
    hal::sleep_ms(PERIOD / 2);

    LED.low();
    hal::sleep_ms(PERIOD / 2);

    unsafe {
        COUNTER += 1;
    }
}
