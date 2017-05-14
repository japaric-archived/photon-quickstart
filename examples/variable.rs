//! Example for the Cloud variable API

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use core::ptr;

use hal::{PinMode, Spark_Data_TypeDef, pin_t};

const D7: pin_t = 7;

app! {
    setup: setup,
    loop: loop_,
}

static mut COUNTER: i32 = 0;

fn setup() {
    unsafe {
        static NAME: &[u8] = b"counter\0";

        hal::HAL_Pin_Mode(D7, PinMode::OUTPUT);

        if hal::spark_variable(
            NAME.as_ptr() as *const i8,
            &COUNTER as *const _ as *const _,
            Spark_Data_TypeDef::CLOUD_VAR_INT,
            ptr::null_mut(),
        )
        {
            hal::HAL_GPIO_Write(D7, 1);
        } else {
            hal::HAL_GPIO_Write(D7, 0);
        }
    }
}

fn loop_() {
    unsafe {
        hal::HAL_GPIO_Write(D7, 0);
        hal::HAL_Delay_Milliseconds(500);
        hal::HAL_GPIO_Write(D7, 1);
        hal::HAL_Delay_Milliseconds(500);
        COUNTER += 1;
    }
}
