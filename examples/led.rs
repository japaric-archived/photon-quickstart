//! Turns ON the user LED

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

app! {
    setup: setup,
    loop: loop_,
}

use hal::gpio::{self, PinMode, pin_t};

const D7: pin_t = 7;

fn setup() {
    unsafe {
        gpio::HAL_Pin_Mode(D7, PinMode::OUTPUT);
        gpio::HAL_GPIO_Write(D7, 1);
    }
}

fn loop_() {}
