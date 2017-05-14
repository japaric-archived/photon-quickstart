//! Transmit the word 'Rust' repeatedly via USB serial.

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

const PERIOD: u32 = 100; // ms
const BAUD_RATE: u32 = 115_200;

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    unsafe {
        hal::USB_USART_Init(BAUD_RATE);
    }
}

fn loop_() {
    unsafe {
        for byte in b"Rust " {
            hal::USB_USART_Send_Data(*byte);
            hal::HAL_Delay_Milliseconds(PERIOD);
        }
    }
}
