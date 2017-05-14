//! Transmit the word 'Rust' repeatedly via USB serial.

#![no_std]

extern crate particle_hal as hal;
#[macro_use]
extern crate photon;

use hal::UsbSerial;

const PERIOD: u32 = 100; // ms
const BAUD_RATE: u32 = 115_200;

app! {
    setup: setup,
    loop: loop_,
}

fn setup() {
    UsbSerial.begin(BAUD_RATE);
}

fn loop_() {
    for byte in b"Rust " {
        UsbSerial.write(*byte);
        hal::sleep_ms(PERIOD);
    }
}
