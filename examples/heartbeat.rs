//! Heartbeat blinking pattern

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::{PinMode, pin_t};

const D7: pin_t = 7;

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    unsafe {
        hal::HAL_Pin_Mode(D7, PinMode::OUTPUT);
    }
}

fn loop_() {
    unsafe {
        hal::HAL_GPIO_Write(D7, 1);
        hal::HAL_Delay_Milliseconds(200);

        hal::HAL_GPIO_Write(D7, 0);
        hal::HAL_Delay_Milliseconds(50);

        hal::HAL_GPIO_Write(D7, 1);
        hal::HAL_Delay_Milliseconds(250);

        hal::HAL_GPIO_Write(D7, 0);
        hal::HAL_Delay_Milliseconds(500);
    }
}
