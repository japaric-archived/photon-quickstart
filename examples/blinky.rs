//! Blinks the user LED

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::delay;
use hal::gpio::{self, PinMode, pin_t};

const D7: pin_t = 7;
const PERIOD: u32 = 250; // ms

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    unsafe {
        gpio::HAL_Pin_Mode(D7, PinMode::OUTPUT);
    }
}

fn loop_() {
    unsafe {
        gpio::HAL_GPIO_Write(D7, 0);
        delay::HAL_Delay_Milliseconds(PERIOD);
        gpio::HAL_GPIO_Write(D7, 1);
        delay::HAL_Delay_Milliseconds(PERIOD);
    }
}
