//! Transmit the word 'Rust' repeatedly via USB serial.

#![no_std]

#[macro_use]
extern crate photon;
extern crate photon_hal as hal;

use hal::UsbSerial;
use photon::App;

const PERIOD: u32 = 100; // ms
const BAUD_RATE: u32 = 115_200;

app! {
    setup: setup,
    loop: loop_,
}

fn setup(_: App) {
    UsbSerial.begin(BAUD_RATE);
}

fn loop_(ref mut app: App) {
    for byte in b"Rust " {
        UsbSerial.write(*byte);
        app.delay_ms(PERIOD);
    }
}
