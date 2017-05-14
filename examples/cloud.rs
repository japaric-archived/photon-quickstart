//! Cloud API example
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

use core::ptr;

use hal::{PinMode, String, pin_t};

const D7: pin_t = 7;

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    unsafe {
        static NAME: &[u8] = b"led\0";

        hal::HAL_Pin_Mode(D7, PinMode::OUTPUT);

        if hal::spark_function(
            NAME.as_ptr() as *const i8,
            toggle,
            ptr::null_mut(),
        )
        {
            hal::HAL_GPIO_Write(D7, 1);
        } else {
            hal::HAL_GPIO_Write(D7, 0);
        }
    }
}

fn loop_() {}

extern "C" fn toggle(command: &String) -> i32 {
    unsafe {
        if &**command == b"on" {
            hal::HAL_GPIO_Write(D7, 1);
            1
        } else if &**command == b"off" {
            hal::HAL_GPIO_Write(D7, 0);
            0
        } else {
            -1
        }
    }
}
